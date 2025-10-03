use crate::server::{
    groups::service::GroupService,
    hosts::{service::HostService, types::base::Host},
    services::{storage::ServiceStorage, types::base::Service},
    shared::types::metadata::TypeMetadataProvider,
    subnets::service::SubnetService,
};
use anyhow::anyhow;
use anyhow::{Error, Result};
use futures::future::try_join_all;
use std::sync::{Arc, OnceLock};
use uuid::Uuid;

pub struct ServiceService {
    storage: Arc<dyn ServiceStorage>,
    subnet_service: Arc<SubnetService>,
    host_service: OnceLock<Arc<HostService>>,
    group_service: Arc<GroupService>,
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
        }
    }

    pub fn set_host_service(&self, host_service: Arc<HostService>) -> Result<(), Arc<HostService>> {
        self.host_service.set(host_service)
    }

    pub async fn create_service(&self, service: Service) -> Result<Service> {
        let host_service = self
            .host_service
            .get()
            .ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;
        let all_hosts = host_service.get_all_hosts().await?;
        let existing_services = self.get_services_for_host(&service.base.host_id).await?;

        let service_from_storage = match existing_services.into_iter().find(|existing: &Service| {
            if let (Some(existing_service_host), Some(new_service_host)) = (
                all_hosts.iter().find(|h| h.id == existing.base.host_id),
                all_hosts.iter().find(|h| h.id == service.base.host_id),
            ) {
                let port_match = new_service_host
                    .base
                    .ports
                    .iter()
                    .any(|p| existing_service_host.base.ports.contains(p));
                let definition_match =
                    service.base.service_definition == existing.base.service_definition;
                return port_match && definition_match;
            }
            false
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
        new_service: Service,
    ) -> Result<Service> {
        let mut interface_updates = 0;
        let mut port_updates = 0;

        for new_service_binding in new_service.base.interface_bindings {
            if !existing_service
                .base
                .interface_bindings
                .contains(&new_service_binding)
            {
                interface_updates += 1;
                existing_service
                    .base
                    .interface_bindings
                    .push(new_service_binding);
            }
        }

        for new_service_port in new_service.base.port_bindings {
            if !existing_service
                .base
                .port_bindings
                .contains(&new_service_port)
            {
                port_updates += 1;
                existing_service.base.port_bindings.push(new_service_port)
            }
        }

        self.storage.update(&existing_service).await?;

        let mut data = Vec::new();

        if port_updates > 0 {
            data.push(format!("{} ports", port_updates))
        };
        if interface_updates > 0 {
            data.push(format!("{} interfaces", interface_updates))
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
            "No new informationt to upsert from service {} to service {}: {}",
            new_service.base.name,
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
        let groups = self.group_service.get_all_groups().await?;

        let group_futures = groups.into_iter().filter_map(|mut group| {
            let initial_bindings_length = group.base.service_bindings.len();

            group.base.service_bindings.retain(|b| {
                // Remove if current service has interface/port bound, updated service doesn't have (or updated service is None aka getting deleted)

                let bindings_exist_after_updates = match updates {
                    Some(updated_service) => {
                        updated_service
                            .base
                            .interface_bindings
                            .contains(&b.interface_id)
                            && updated_service.base.port_bindings.contains(&b.port_id)
                    }
                    None => false,
                };

                !((current_service
                    .base
                    .interface_bindings
                    .contains(&b.interface_id)
                    || current_service.base.port_bindings.contains(&b.port_id))
                    && !bindings_exist_after_updates)
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
        service.base.interface_bindings = service
            .base
            .interface_bindings
            .iter()
            .filter_map(|b| {
                if let Some(original_binding) = original_host.get_interface(b) {
                    return new_host.base.interfaces.iter().find_map(|i| {
                        if i == original_binding {
                            Some(i.id)
                        } else {
                            None
                        }
                    });
                }
                None
            })
            .collect();

        service.base.port_bindings = service
            .base
            .port_bindings
            .iter()
            .filter_map(|b| {
                if let Some(original_binding) = original_host.get_port(b) {
                    return new_host.base.ports.iter().find_map(|p| {
                        if p == original_binding {
                            Some(p.id)
                        } else {
                            None
                        }
                    });
                }
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
