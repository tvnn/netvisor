use anyhow::{anyhow, Error, Result};
use futures::future::try_join_all;
use itertools::Itertools;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    hosts::{

        storage::HostStorage,
        types::{
            base::{Host, HostBase}
        }
    }, services::{service::ServiceService}, subnets::service::SubnetService
};

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    subnet_service: Arc<SubnetService>,
    service_service: Arc<ServiceService>,
}

impl HostService {
    pub fn new(storage: Arc<dyn HostStorage>, subnet_service: Arc<SubnetService>, service_service: Arc<ServiceService>) -> Self {
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
                tracing::warn!("Duplicate host for {}: {} found, {}: {} - upserting discovery data...", host.base.name, host.id, existing_host.base.name, existing_host.id);
                self.update_subnet_host_relationships(&existing_host, true).await?;
                self.upsert_host(existing_host, host).await?
            }
            None => {
                self.storage.create(&host).await?;
                tracing::info!("Created host {}: {}", host.base.name, host.id);
                host
            }
        };

        self.update_subnet_host_relationships(&host_from_storage, false).await?;

        Ok(host_from_storage)
    }

    pub async fn update_host(&self, mut host: Host) -> Result<Host, Error> {
        
        let current_host = self.get_host(&host.id).await?.ok_or_else(||anyhow!("Host '{}' not found", host.id))?;

        self.update_subnet_host_relationships(&current_host, true).await?;
        self.update_subnet_host_relationships(&host, false).await?;
        
        host.updated_at = chrono::Utc::now();

        self.storage.update(&host).await?;
        Ok(host)
    }

    /// Merge new discovery data with existing host
    async fn upsert_host(&self, mut existing_host: Host, new_host: Host) -> Result<Host> {

        // Merge interfaces - add any new interfaces not already present
        for new_host_interface in new_host.base.interfaces {
            if !existing_host.base.interfaces.iter().any(|existing| *existing == new_host_interface) {
                existing_host.base.interfaces.push(new_host_interface);
            }
        }

        // Merge open ports - add any new ports not already present
        for new_port in new_host.base.ports {
            if !existing_host.base.ports.iter().any(|existing| *existing == new_port) {
                existing_host.base.ports.push(new_port);
            }
        }

        existing_host.base.services = [new_host.base.services, existing_host.base.services].concat();

        // Update other fields if they have more information
        if existing_host.base.hostname.is_none() && new_host.base.hostname.is_some() {
            existing_host.base.hostname = new_host.base.hostname;
        }
        
        if existing_host.base.description.is_none() && new_host.base.description.is_some() {
            existing_host.base.description = new_host.base.description;
        }

        existing_host.updated_at = chrono::Utc::now();

        // Update the existing host
        self.storage.update(&existing_host).await?;
        tracing::info!("Upserted host {}: {} with new discovery data", existing_host.base.name, existing_host.id);
        
        Ok(existing_host)
    }


    pub async fn consolidate_hosts(&self, destination_host: Host, other_host: Host) -> Result<Host> {
        let other_host_services = self.service_service.get_services_for_host(&other_host.id).await?;
        let (other_host_name, other_host_id) = (&other_host.base.name, &other_host.id);

        let updated_host = self.upsert_host(destination_host, other_host.clone()).await?;

        let service_update_futures = other_host_services.into_iter().map(|mut s| {
            s = self.service_service.transfer_service_to_new_host(&mut s, &other_host, &updated_host);
            self.service_service.update_service(s)
        });

        try_join_all(service_update_futures).await?;

        self.delete_host(&other_host_id, true).await?;
        tracing::info!("Consolidated host {}: {} into {}: {}", other_host_name, other_host_id, updated_host.base.name, updated_host.id);
        Ok(updated_host)
    }

    pub async fn update_subnet_host_relationships(&self, host: &Host, remove: bool) -> Result<(), Error>{
        let subnet_ids: Vec<Uuid> = host.base.interfaces.iter().map(|i| i.base.subnet_id).unique().collect();
        
        if let Ok(mut subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
            let subnet_futures: Vec<_> = subnets
                .iter_mut()
                .map(|subnet| {
                        
                        if remove { subnet.remove_host_relationship(host) }
                        else { subnet.create_host_relationship(host) };
                        
                        return self.subnet_service.update_subnet(subnet.clone())
                    }
                )
                .collect();

            try_join_all(subnet_futures).await?;
        };
        Ok(())
    }

    pub async fn delete_host(&self, id: &Uuid, ignore_services: bool) -> Result<()> {

        let host = self.get_host(id).await?.ok_or_else(|| anyhow::anyhow!("Host {} not found", id))?;

        if !ignore_services {
            for service_id in &host.base.services{
                self.service_service.delete_service(service_id, false).await?;

            };
        }
        
        self.update_subnet_host_relationships(&host, true).await?;

        self.storage.delete(id).await?;
        tracing::info!("Deleted host {}: {}; deleted services: {}", host.base.name, host.id, !ignore_services);
        Ok(())
    }
}