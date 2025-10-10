use crate::server::{
    groups::service::GroupService,
    hosts::{
        service::HostService,
        types::{base::Host, interfaces::Interface, ports::Port},
    },
    services::{
        storage::ServiceStorage,
        types::{base::Service, bindings::Binding},
    },
    shared::types::metadata::TypeMetadataProvider,
    subnets::service::SubnetService,
};
use anyhow::anyhow;
use anyhow::{Error, Result};
use futures::{future::try_join_all, lock::Mutex};
use std::sync::{Arc, OnceLock};
use uuid::Uuid;

pub struct ServiceService {
    storage: Arc<dyn ServiceStorage>,
    subnet_service: Arc<SubnetService>,
    host_service: OnceLock<Arc<HostService>>,
    group_service: Arc<GroupService>,
    subnet_update_lock: Arc<Mutex<()>>,
    group_update_lock: Arc<Mutex<()>>,
}

impl ServiceService {
    pub fn new(
        storage: Arc<dyn ServiceStorage>,
        subnet_service: Arc<SubnetService>,
        group_service: Arc<GroupService>,
    ) -> Self {
        Self {
            storage,
            subnet_service,
            group_service,
            host_service: OnceLock::new(),
            subnet_update_lock: Arc::new(Mutex::new(())),
            group_update_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn set_host_service(&self, host_service: Arc<HostService>) -> Result<(), Arc<HostService>> {
        self.host_service.set(host_service)
    }

    pub async fn create_service(&self, service: Service) -> Result<Service> {
        let existing_services = self.get_services_for_host(&service.base.host_id).await?;

        let service_from_storage = match existing_services.into_iter().find(|existing: &Service| {
            // Must be same host and same definition
            let host_match = existing.base.host_id == service.base.host_id;
            let definition_match =
                service.base.service_definition == existing.base.service_definition;

            if !host_match || !definition_match {
                return false;
            }

            // Check if bindings overlap
            let bindings_match = existing.base.bindings.iter().any(|existing_binding| {
                service.base.bindings.iter().any(|service_binding| {
                    match (existing_binding, service_binding) {
                        // L4 bindings match if they share the same port
                        (
                            Binding::Layer4 {
                                port_id: existing_port,
                                ..
                            },
                            Binding::Layer4 {
                                port_id: service_port,
                                ..
                            },
                        ) => existing_port == service_port,

                        // L3 bindings match if they share the same interface
                        (
                            Binding::Layer3 {
                                interface_id: existing_iface,
                                ..
                            },
                            Binding::Layer3 {
                                interface_id: service_iface,
                                ..
                            },
                        ) => existing_iface == service_iface,

                        // L3 and L4 bindings never match each other
                        _ => false,
                    }
                })
            });

            bindings_match
        }) {
            Some(existing_service) => {
                tracing::warn!(
                    "Duplicate service for {}: {} found, {}: {} - upserting discovery data...",
                    service.base.name,
                    service.id,
                    existing_service.base.name,
                    existing_service.id
                );
                self.upsert_service(existing_service, service).await?
            }
            None => {
                self.storage.create(&service).await?;
                self.update_subnet_service_bindings(None, Some(&service))
                    .await?;
                tracing::info!(
                    "Created service {} for host {}",
                    service.base.service_definition.name(),
                    service.base.host_id
                );
                service
            }
        };

        Ok(service_from_storage)
    }

    pub async fn upsert_service(
        &self,
        mut existing_service: Service,
        new_service_data: Service,
    ) -> Result<Service> {
        let mut binding_updates = 0;

        for new_service_binding in new_service_data.base.bindings {
            if !existing_service
                .base
                .bindings
                .contains(&new_service_binding)
            {
                binding_updates += 1;
                existing_service.base.bindings.push(new_service_binding);
            }
        }

        self.storage.update(&existing_service).await?;

        let mut data = Vec::new();

        if binding_updates > 0 {
            data.push(format!("{} bindings", binding_updates))
        };

        if !data.is_empty() {
            tracing::info!(
                "Upserted service {}: {} with new data: {}",
                existing_service.base.name,
                existing_service.id,
                data.join(", ")
            );
        }
        tracing::info!(
            "No new information to upsert from service {} to service {}: {}",
            new_service_data.base.name,
            existing_service.base.name,
            existing_service.id
        );

        Ok(existing_service)
    }

    pub async fn get_service(&self, id: &Uuid) -> Result<Option<Service>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_services(&self) -> Result<Vec<Service>> {
        self.storage.get_all().await
    }

    pub async fn get_services_for_host(&self, host_id: &Uuid) -> Result<Vec<Service>> {
        self.storage.get_services_for_host(host_id).await
    }

    pub async fn update_service(&self, mut service: Service) -> Result<Service> {
        let current_service = self
            .get_service(&service.id)
            .await?
            .ok_or_else(|| anyhow!("Could not find service"))?;

        self.update_group_service_bindings(&current_service, Some(&service))
            .await?;
        self.update_subnet_service_bindings(Some(&current_service), Some(&service))
            .await?;

        service.updated_at = chrono::Utc::now();

        self.storage.update(&service).await?;
        tracing::info!(
            "Updated service {}: {} for host {}",
            service.base.name,
            service.id,
            service.base.host_id
        );
        Ok(service)
    }

    pub async fn update_group_service_bindings(
        &self,
        current_service: &Service,
        updates: Option<&Service>,
    ) -> Result<(), Error> {
        let _guard = self.group_update_lock.lock().await;

        let groups = self.group_service.get_all_groups().await?;

        let group_futures = groups.into_iter().filter_map(|mut group| {
            let initial_bindings_length = group.base.service_bindings.len();

            group.base.service_bindings.retain(|sb| {
                // Remove if updated service doesn't have binding (or updated service is None aka getting deleted)
                if sb.service_id == current_service.id {
                    return match updates {
                        Some(updated_service) => updated_service
                            .base
                            .bindings
                            .iter()
                            .any(|pb| pb.id() == sb.binding_id),
                        None => false,
                    };
                }
                true
            });

            if group.base.service_bindings.len() != initial_bindings_length {
                return Some(self.group_service.update_group(group));
            }
            None
        });

        try_join_all(group_futures).await?;

        Ok(())
    }

    pub async fn update_subnet_service_bindings(
        &self,
        current_service: Option<&Service>,
        updates: Option<&Service>,
    ) -> Result<(), Error> {
        let host_service = self
            .host_service
            .get()
            .ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;

        let host = match updates {
            Some(updated_service) => Some(
                host_service
                    .get_host(&updated_service.base.host_id)
                    .await?
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Could not find host for service {}: {}",
                            updated_service.base.name,
                            updated_service.id
                        )
                    })?,
            ),
            None => None,
        };

        let _guard = self.subnet_update_lock.lock().await;

        let subnets = self.subnet_service.get_all_subnets().await?;

        let subnet_futures = subnets.into_iter().filter_map(|mut subnet| {
            let initial_dns = subnet.base.dns_resolvers.clone();
            let initial_gateways = subnet.base.gateways.clone();
            let initial_reverse_proxies = subnet.base.reverse_proxies.clone();

            if let Some(current) = current_service {
                subnet.remove_service_relationships(current);
            }

            if let (Some(updated_service), Some(h)) = (updates, &host) {
                subnet.create_service_relationships(updated_service, h);
            }

            let new_dns = subnet.base.dns_resolvers.clone();
            let new_gateways = subnet.base.gateways.clone();
            let new_reverse_proxies = subnet.base.reverse_proxies.clone();

            if initial_dns != new_dns
                || initial_gateways != new_gateways
                || initial_reverse_proxies != new_reverse_proxies
            {
                return Some(self.subnet_service.update_subnet(subnet));
            }
            None
        });

        try_join_all(subnet_futures).await?;

        Ok(())
    }

    pub fn transfer_service_to_new_host(
        &self,
        service: &mut Service,
        original_host: &Host,
        new_host: &Host,
    ) -> Service {
        service.base.bindings = service
            .base
            .bindings
            .iter()
            .filter_map(|b| {
                let original_interface = original_host.get_interface(&b.interface_id());

                match b {
                    Binding::Layer3 { .. } => {
                        if let Some(original_interface) = original_interface {
                            let new_interface: Option<&Interface> = new_host
                                .base
                                .interfaces
                                .iter()
                                .find(|i| *i == original_interface);

                            if let Some(new_interface) = new_interface {
                                return Some(Binding::new_l3(new_interface.id));
                            }
                        }
                    }
                    Binding::Layer4 { port_id, .. } => {
                        let original_port = original_host.get_port(port_id);

                        let new_port: Option<&Port> = if let Some(port) = original_port {
                            new_host.base.ports.iter().find(|p| *p == port)
                        } else {
                            None
                        };

                        let new_interface: Option<&Interface> =
                            if let Some(interface) = original_interface {
                                new_host.base.interfaces.iter().find(|i| *i == interface)
                            } else {
                                None
                            };

                        match (new_port, new_interface) {
                            (Some(new_port), Some(new_interface)) => {
                                return Some(Binding::new_l4(new_port.id, Some(new_interface.id)));
                            }
                            (Some(new_port), None) if b.interface_id().is_none() => {
                                return Some(Binding::new_l4(new_port.id, None));
                            }
                            _ => return None,
                        }
                    }
                };

                None
            })
            .collect();

        service.base.host_id = new_host.id;

        service.clone()
    }

    pub async fn delete_service(&self, id: &Uuid) -> Result<()> {
        let service = self
            .get_service(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Service {} not found", id))?;

        self.update_group_service_bindings(&service, None).await?;
        self.update_subnet_service_bindings(Some(&service), None)
            .await?;

        self.storage.delete(id).await?;
        tracing::info!(
            "Deleted service {}: {} for host {}",
            service.base.name,
            service.id,
            service.base.host_id
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        server::services::types::{
            bindings::{Binding, ServiceBinding},
            definitions::ServiceDefinitionExt,
        },
        tests::*,
    };

    #[tokio::test]
    async fn test_service_deduplication_on_create() {
        let (_, services) = test_services().await;

        let subnet_obj = subnet();
        services
            .subnet_service
            .create_subnet(subnet_obj.clone())
            .await
            .unwrap();

        // Create first service + host
        let mut host_obj = host();
        host_obj.base.interfaces = vec![interface(&subnet_obj.id)];

        let mut svc1 = service(&host_obj.id);
        // Add bindings so the deduplication logic can match them
        svc1.base.bindings = vec![Binding::new_l4(
            host_obj.base.ports[0].id,
            Some(host_obj.base.interfaces[0].id),
        )];

        let (created_host, created1) = services
            .host_service
            .create_host_with_services(host_obj.clone(), vec![svc1.clone()])
            .await
            .unwrap();

        // Try to create duplicate (same definition + matching bindings)
        // Must use created_host's IDs since host deduplication may have changed them
        let mut svc2 = service(&created_host.id);
        svc2.base.service_definition = svc1.base.service_definition.clone();
        svc2.base.bindings = vec![Binding::new_l4(
            created_host.base.ports[0].id,
            Some(created_host.base.interfaces[0].id),
        )];

        let created2 = services
            .service_service
            .create_service(svc2.clone())
            .await
            .unwrap();

        // Should return same service (upserted)
        assert_eq!(created1[0].id, created2.id);

        // Verify only one service in DB
        let all_services = services
            .service_service
            .get_services_for_host(&created_host.id)
            .await
            .unwrap();
        assert_eq!(all_services.len(), 1);
    }

    #[tokio::test]
    async fn test_service_update_maintains_subnet_relationships() {
        let (_, services) = test_services().await;

        let subnet_obj = subnet();
        let created_subnet = services
            .subnet_service
            .create_subnet(subnet_obj.clone())
            .await
            .unwrap();

        let mut host_obj = host();
        host_obj.base.interfaces = vec![interface(&created_subnet.id)];

        // Create reverse proxy service (will add to subnet reverse proxies)
        let reverse_proxy_def =
            crate::server::services::definitions::ServiceDefinitionRegistry::find_by_id(
                "Nginx Proxy Manager",
            )
            .unwrap();
        let mut reverse_proxy_svc = service(&host_obj.id);
        reverse_proxy_svc.base.service_definition = reverse_proxy_def;
        reverse_proxy_svc.base.bindings = vec![Binding::new_l4(
            host_obj.base.ports[0].id,
            Some(host_obj.base.interfaces[0].id),
        )];

        // Create gateway service (will add to subnet gateways)
        let gateway_def =
            crate::server::services::definitions::ServiceDefinitionRegistry::find_by_id("Gateway")
                .unwrap();
        let mut gateway_svc = service(&host_obj.id);
        gateway_svc.base.service_definition = gateway_def;
        gateway_svc.base.bindings = vec![Binding::new_l3(host_obj.base.interfaces[0].id)];

        let (_, created_svcs) = services
            .host_service
            .create_host_with_services(host_obj.clone(), vec![reverse_proxy_svc, gateway_svc])
            .await
            .unwrap();

        // Subnet should have both gateway & reverse proxy relationships
        let subnet_after = services
            .subnet_service
            .get_subnet(&created_subnet.id)
            .await
            .unwrap()
            .unwrap();

        println!("{:?}", subnet_after);

        // Find which service is which by their definition
        let reverse_proxy_svc = created_svcs
            .iter()
            .find(|s| s.base.service_definition.is_reverse_proxy())
            .expect("Reverse proxy service not found");
        let gateway_svc = created_svcs
            .iter()
            .find(|s| s.base.service_definition.is_gateway())
            .expect("Gateway service not found");

        assert!(!subnet_after.base.reverse_proxies.is_empty());
        assert_eq!(
            subnet_after.base.reverse_proxies[0].service_id,
            reverse_proxy_svc.id
        );

        assert!(!subnet_after.base.gateways.is_empty());
        assert_eq!(subnet_after.base.gateways[0].service_id, gateway_svc.id);
    }

    #[tokio::test]
    async fn test_service_deletion_cleans_up_relationships() {
        let (_, services) = test_services().await;

        let subnet_obj = subnet();
        let created_subnet = services
            .subnet_service
            .create_subnet(subnet_obj.clone())
            .await
            .unwrap();

        let mut host_obj = host();
        host_obj.base.interfaces = vec![interface(&created_subnet.id)];

        // Create service in a group
        let mut svc = service(&host_obj.id);
        let binding = Binding::new_l4(
            host_obj.base.ports[0].id,
            Some(host_obj.base.interfaces[0].id),
        );
        svc.base.bindings = vec![binding];

        let (_, created_svcs) = services
            .host_service
            .create_host_with_services(host_obj.clone(), vec![svc])
            .await
            .unwrap();
        let created_svc = &created_svcs[0];

        let mut group_obj = group();
        group_obj.base.service_bindings = vec![ServiceBinding {
            service_id: created_svc.id,
            binding_id: created_svc.base.bindings[0].id(),
        }];
        let created_group = services
            .group_service
            .create_group(group_obj)
            .await
            .unwrap();

        // Delete service
        services
            .service_service
            .delete_service(&created_svc.id)
            .await
            .unwrap();

        // Group should no longer have service binding
        let group_after = services
            .group_service
            .get_group(&created_group.id)
            .await
            .unwrap()
            .unwrap();
        assert!(group_after.base.service_bindings.is_empty());
    }
}
