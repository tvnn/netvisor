use crate::server::{
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
use uuid::Uuid;

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    subnet_service: Arc<SubnetService>,
    service_service: Arc<ServiceService>,
}

impl HostService {
    pub fn new(
        storage: Arc<dyn HostStorage>,
        subnet_service: Arc<SubnetService>,
        service_service: Arc<ServiceService>,
    ) -> Self {
        Self {
            storage,
            subnet_service,
            service_service,
        }
    }

    pub async fn get_host(&self, id: &Uuid) -> Result<Option<Host>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_hosts(&self) -> Result<Vec<Host>> {
        self.storage.get_all().await
    }

    /// Create a new host
    pub async fn create_host(&self, host_base: HostBase) -> Result<Host> {
        let host = Host::new(host_base);

        let all_hosts = self.storage.get_all().await?;

        let host_from_storage = match all_hosts.into_iter().find(|h| host.eq(h)) {
            Some(existing_host) => {
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
            None => {
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

        self.update_services(&current_host, &host).await?;

        self.update_subnet_host_relationships(&current_host, true)
            .await?;
        self.update_subnet_host_relationships(&host, false).await?;

        host.updated_at = chrono::Utc::now();

        self.storage.update(&host).await?;
        Ok(host)
    }

    /// Merge new discovery data with existing host
    async fn upsert_host(&self, mut existing_host: Host, new_host: Host) -> Result<Host> {
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
        let other_host_services = self
            .service_service
            .get_services_for_host(&other_host.id)
            .await?;
        let (other_host_name, other_host_id) = (&other_host.base.name, &other_host.id);

        let updated_host = self
            .upsert_host(destination_host, other_host.clone())
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

    pub async fn update_services(&self, current_host: &Host, updates: &Host) -> Result<(), Error> {
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
