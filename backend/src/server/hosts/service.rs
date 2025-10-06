use crate::server::{
    daemons::service::DaemonService,
    discovery::types::base::EntitySourceDiscriminants,
    hosts::{
        storage::HostStorage,
        types::base::{Host, HostBase},
    },
    services::{service::ServiceService, types::base::Service},
    subnets::service::SubnetService,
};
use anyhow::{anyhow, Error, Result};
use futures::future::try_join_all;
use itertools::Itertools;
use std::sync::Arc;
use strum::IntoDiscriminant;
use uuid::Uuid;

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    subnet_service: Arc<SubnetService>,
    service_service: Arc<ServiceService>,
    daemon_service: Arc<DaemonService>,
}

impl HostService {
    pub fn new(
        storage: Arc<dyn HostStorage>,
        subnet_service: Arc<SubnetService>,
        service_service: Arc<ServiceService>,
        daemon_service: Arc<DaemonService>,
    ) -> Self {
        Self {
            storage,
            subnet_service,
            service_service,
            daemon_service,
        }
    }

    pub async fn get_host(&self, id: &Uuid) -> Result<Option<Host>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_hosts(&self) -> Result<Vec<Host>> {
        self.storage.get_all().await
    }

    pub async fn create_host_with_services(
        &self,
        host: Host,
        services: Vec<Service>,
    ) -> Result<(Host, Vec<Service>)> {
        // Create host first (handles duplicates via upsert_host)
        let mut created_host = self.create_host(&host.base).await?;

        // Create services, handling case where created_host was upserted instead of created anew, which means that host ID + interfaces/port IDs
        // are different from what's mapped to the service and they need to be updated
        let service_futures = services.into_iter().map(|mut service| {
            service = self.service_service.transfer_service_to_new_host(
                &mut service,
                &host,
                &created_host,
            );
            self.service_service.create_service(service)
        });

        let services = try_join_all(service_futures).await?;

        // Add all successfully created/found services to the host
        for service in &services {
            if !created_host.base.services.contains(&service.id) {
                created_host.base.services.push(service.id);
            }
        }

        let host_with_final_services = self.update_host(created_host).await?;

        Ok((host_with_final_services, services))
    }

    /// Create a new host
    async fn create_host(&self, host_base: &HostBase) -> Result<Host> {
        let host = Host::new(host_base.clone());

        let all_hosts = self.storage.get_all().await?;

        let host_from_storage = match all_hosts.into_iter().find(|h| host.eq(h)) {
            Some(existing_host)
                if host.base.source.discriminant() == EntitySourceDiscriminants::Discovery =>
            {
                tracing::warn!(
                    "Duplicate host for {}: {} found, {}: {} - upserting discovery data...",
                    host.base.name,
                    host.id,
                    existing_host.base.name,
                    existing_host.id
                );
                self.update_subnet_host_relationships(&existing_host, true)
                    .await?;
                self.upsert_host(existing_host, host).await?
            }
            _ => {
                self.storage.create(&host).await?;
                tracing::info!("Created host {}: {}", host.base.name, host.id);
                host
            }
        };

        self.update_subnet_host_relationships(&host_from_storage, false)
            .await?;

        Ok(host_from_storage)
    }

    pub async fn update_host(&self, mut host: Host) -> Result<Host, Error> {
        let current_host = self
            .get_host(&host.id)
            .await?
            .ok_or_else(|| anyhow!("Host '{}' not found", host.id))?;

        self.update_host_services(&current_host, &host).await?;

        self.update_subnet_host_relationships(&current_host, true)
            .await?;
        self.update_subnet_host_relationships(&host, false).await?;

        host.updated_at = chrono::Utc::now();

        self.storage.update(&host).await?;
        Ok(host)
    }

    /// Merge new discovery data with existing host
    async fn upsert_host(&self, mut existing_host: Host, new_host: Host) -> Result<Host> {
        if existing_host.id == new_host.id {
            return Err(anyhow!("Can't upsert a host with itself"));
        }

        let mut interface_updates = 0;
        let mut port_updates = 0;
        let mut hostname_update = false;
        let mut description_update = false;

        // Merge interfaces - add any new interfaces not already present
        for new_host_interface in new_host.base.interfaces {
            if !existing_host.base.interfaces.contains(&new_host_interface) {
                interface_updates += 1;
                existing_host.base.interfaces.push(new_host_interface);
            }
        }

        // Merge open ports - add any new ports not already present
        for new_port in new_host.base.ports {
            if !existing_host.base.ports.contains(&new_port) {
                port_updates += 1;
                existing_host.base.ports.push(new_port);
            }
        }

        existing_host.base.services =
            [new_host.base.services, existing_host.base.services].concat();

        // Update other fields if they have more information
        if existing_host.base.hostname.is_none() && new_host.base.hostname.is_some() {
            hostname_update = true;
            existing_host.base.hostname = new_host.base.hostname;
        }

        if existing_host.base.description.is_none() && new_host.base.description.is_some() {
            description_update = true;
            existing_host.base.description = new_host.base.description;
        }

        existing_host.updated_at = chrono::Utc::now();

        // Update the existing host
        self.storage.update(&existing_host).await?;
        let mut data = Vec::new();

        if port_updates > 0 {
            data.push(format!("{} ports", port_updates))
        };
        if interface_updates > 0 {
            data.push(format!("{} interfaces", interface_updates))
        };
        if hostname_update {
            data.push("new hostname".to_string())
        }
        if description_update {
            data.push("new description".to_string())
        }

        if !data.is_empty() {
            tracing::info!(
                "Upserted host {}: {} with new discovery data: {}",
                existing_host.base.name,
                existing_host.id,
                data.join(", ")
            );
        }
        tracing::info!(
            "No new informationt to upsert from host {} to host {}: {}",
            new_host.base.name,
            existing_host.base.name,
            existing_host.id
        );

        Ok(existing_host)
    }

    pub async fn consolidate_hosts(
        &self,
        destination_host: Host,
        other_host: Host,
    ) -> Result<Host> {
        if destination_host.id == other_host.id {
            return Err(anyhow!("Can't consolidate a host with itself"));
        }

        if self
            .daemon_service
            .get_host_daemon(&other_host.id)
            .await?
            .is_some()
        {
            return Err(anyhow!("Can't consolidate a host that has a daemon. Consolidate the other host into the daemon host."));
        }

        let other_host_services = self
            .service_service
            .get_services_for_host(&other_host.id)
            .await?;
        let (other_host_name, other_host_id) = (&other_host.base.name, &other_host.id);

        let updated_host = self
            .upsert_host(destination_host.clone(), other_host.clone())
            .await?;

        let service_update_futures = other_host_services.into_iter().map(|mut s| {
            s = self.service_service.transfer_service_to_new_host(
                &mut s,
                &other_host,
                &updated_host,
            );

            self.service_service.update_service(s)
        });

        try_join_all(service_update_futures).await?;

        // Ignore services because they are just being moved to other host
        self.delete_host(other_host_id, false).await?;
        tracing::info!(
            "Consolidated host {}: {} into {}: {}",
            other_host_name,
            other_host_id,
            updated_host.base.name,
            updated_host.id
        );
        Ok(updated_host)
    }

    pub async fn update_host_services(
        &self,
        current_host: &Host,
        updates: &Host,
    ) -> Result<(), Error> {
        let services = self
            .service_service
            .get_services_for_host(&current_host.id)
            .await?;

        let (update_services, delete_services): (Vec<Service>, Vec<Service>) = services
            .into_iter()
            .partition(|s| updates.base.services.contains(&s.id));

        let delete_service_futures = delete_services
            .iter()
            .map(|s| self.service_service.delete_service(&s.id));

        try_join_all(delete_service_futures).await?;

        let update_service_futures = update_services.into_iter().filter_map(|mut service| {
            let initial_interface_bindings_count = service.base.interface_bindings.len();
            let initial_port_bindings_count = service.base.port_bindings.len();

            service.base.interface_bindings.retain(|b| {
                // Remove if current host has interface, updated host doesn't have
                !(current_host.get_interface(b).is_some() && updates.get_interface(b).is_none())
            });

            service.base.port_bindings.retain(|b| {
                // Remove if current host has port, updated host doesn't have
                !(current_host.get_port(b).is_some() && updates.get_port(b).is_none())
            });

            if initial_interface_bindings_count != service.base.interface_bindings.len()
                || initial_port_bindings_count != service.base.port_bindings.len()
            {
                return Some(self.service_service.update_service(service));
            }
            None
        });

        try_join_all(update_service_futures).await?;

        Ok(())
    }

    pub async fn update_subnet_host_relationships(
        &self,
        host: &Host,
        remove: bool,
    ) -> Result<(), Error> {
        let subnet_ids: Vec<Uuid> = host
            .base
            .interfaces
            .iter()
            .map(|i| i.base.subnet_id)
            .unique()
            .collect();

        if let Ok(mut subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
            let subnet_futures: Vec<_> = subnets
                .iter_mut()
                .map(|subnet| {
                    if remove {
                        subnet.remove_host_relationship(host)
                    } else {
                        subnet.create_host_relationship(host)
                    };

                    self.subnet_service.update_subnet(subnet.clone())
                })
                .collect();

            try_join_all(subnet_futures).await?;
        };
        Ok(())
    }

    pub async fn delete_host(&self, id: &Uuid, delete_services: bool) -> Result<()> {
        let host = self
            .get_host(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Host {} not found", id))?;

        if self.daemon_service.get_host_daemon(id).await?.is_some() {
            return Err(anyhow!("Can't delete a host that has a daemon."));
        }

        if delete_services {
            for service_id in &host.base.services {
                self.service_service.delete_service(service_id).await?;
            }
        }

        self.update_subnet_host_relationships(&host, true).await?;

        self.storage.delete(id).await?;
        tracing::info!(
            "Deleted host {}: {}; deleted service + associated subnet/group bindings: {}",
            host.base.name,
            host.id,
            !delete_services
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::tests::*;

    #[tokio::test]
    async fn test_host_deduplication_on_create() {
        let (storage, services) = test_services().await;

        let start_host_count = storage.hosts.get_all().await.unwrap().len();

        // Create first host
        let host1 = host();
        let (created1, _) = services
            .host_service
            .create_host_with_services(host1.clone(), vec![])
            .await
            .unwrap();

        // Try to create duplicate (same interfaces)
        let host2 = host();
        let (created2, _) = services
            .host_service
            .create_host_with_services(host2.clone(), vec![])
            .await
            .unwrap();

        // Should return same host (upserted)
        assert_eq!(created1.id, created2.id);

        // Verify only one host in DB
        let end_host_count = storage.hosts.get_all().await.unwrap().len();
        assert_eq!(start_host_count + 1, end_host_count);
    }

    #[tokio::test]
    async fn test_host_upsert_merges_new_data() {
        let (_, services) = test_services().await;

        // Create host with one interface
        let mut host1 = host();
        let subnet1 = subnet();
        services
            .subnet_service
            .create_subnet(subnet1.clone())
            .await
            .unwrap();
        host1.base.interfaces = vec![interface(&subnet1.id)];

        let (created, _) = services
            .host_service
            .create_host_with_services(host1.clone(), vec![])
            .await
            .unwrap();

        // Create "duplicate" with additional interface
        let mut host2 = host();
        let subnet2 = subnet();
        services
            .subnet_service
            .create_subnet(subnet2.clone())
            .await
            .unwrap();
        host2.base.interfaces = vec![interface(&subnet1.id), interface(&subnet2.id)];

        let (upserted, _) = services
            .host_service
            .create_host_with_services(host2.clone(), vec![])
            .await
            .unwrap();

        // Should have merged interfaces
        assert_eq!(upserted.id, created.id);
        assert_eq!(upserted.base.interfaces.len(), 2);
    }

    #[tokio::test]
    async fn test_host_consolidation() {
        let (_, services) = test_services().await;

        let subnet_obj = subnet();
        services
            .subnet_service
            .create_subnet(subnet_obj.clone())
            .await
            .unwrap();

        let mut host1 = host();
        host1.base.interfaces = Vec::new();

        let (created1, _) = services
            .host_service
            .create_host_with_services(host1.clone(), vec![])
            .await
            .unwrap();

        let mut host2 = host();
        host2.base.interfaces = vec![interface(&subnet_obj.id)];

        let mut svc = service(&host2.id);
        svc.base.port_bindings = vec![host2.base.ports[0].id];
        svc.base.interface_bindings = vec![host2.base.interfaces[0].id];

        let (created2, created_svcs) = services
            .host_service
            .create_host_with_services(host2.clone(), vec![svc])
            .await
            .unwrap();

        let created_svc = &created_svcs[0];

        // Consolidate host2 into host1
        let consolidated = services
            .host_service
            .consolidate_hosts(created1.clone(), created2.clone())
            .await
            .unwrap();

        // Host1 should have host2's service
        assert!(consolidated.base.services.contains(&created_svc.id));

        // Host2 should be deleted
        let host2_after = services.host_service.get_host(&created2.id).await.unwrap();
        assert!(host2_after.is_none());

        // Service should now belong to host1
        let svc_after = services
            .service_service
            .get_service(&created_svc.id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(svc_after.base.host_id, consolidated.id);
    }

    #[tokio::test]
    async fn test_host_deletion_removes_subnet_relationships() {
        let (_, services) = test_services().await;

        let subnet_obj = subnet();
        let created_subnet = services
            .subnet_service
            .create_subnet(subnet_obj.clone())
            .await
            .unwrap();

        // Create host with interface on subnet
        let mut host_obj = host();
        host_obj.base.interfaces = vec![interface(&created_subnet.id)];
        let (created_host, _) = services
            .host_service
            .create_host_with_services(host_obj.clone(), vec![])
            .await
            .unwrap();

        // Subnet should have host relationship
        let subnet_after_create = services
            .subnet_service
            .get_subnet(&created_subnet.id)
            .await
            .unwrap()
            .unwrap();
        assert!(subnet_after_create.base.hosts.contains(&created_host.id));

        // Delete host (with services)
        services
            .host_service
            .delete_host(&created_host.id, true)
            .await
            .unwrap();

        // Subnet should no longer have host relationship
        let subnet_after_delete = services
            .subnet_service
            .get_subnet(&created_subnet.id)
            .await
            .unwrap()
            .unwrap();
        assert!(!subnet_after_delete.base.hosts.contains(&created_host.id));
    }
}
