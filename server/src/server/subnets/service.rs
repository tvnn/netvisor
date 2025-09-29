use crate::server::{
    hosts::service::HostService,
    subnets::{storage::SubnetStorage, types::base::Subnet},
};
use anyhow::Result;
use futures::future::try_join_all;
use std::sync::{Arc, OnceLock};
use uuid::Uuid;

pub struct SubnetService {
    storage: Arc<dyn SubnetStorage>,
    host_service: OnceLock<Arc<HostService>>,
}

impl SubnetService {
    pub fn new(storage: Arc<dyn SubnetStorage>) -> Self {
        Self {
            storage,
            host_service: OnceLock::new(),
        }
    }

    pub fn set_host_service(&self, host_service: Arc<HostService>) -> Result<(), Arc<HostService>> {
        self.host_service.set(host_service)
    }

    /// Create a new subnet
    pub async fn create_subnet(&self, subnet: Subnet) -> Result<Subnet> {
        let all_subnets = self.storage.get_all().await?;

        let subnet_from_storage = match all_subnets.iter().find(|s| subnet.eq(s)) {
            Some(existing_subnet) => {
                tracing::warn!(
                    "Duplicate subnet for {}: {} found, returning existing {}: {}",
                    subnet.base.name,
                    subnet.id,
                    existing_subnet.base.name,
                    existing_subnet.id
                );
                existing_subnet.clone()
            }
            None => {
                self.storage.create(&subnet).await?;
                tracing::info!("Created subnet {}: {}", subnet.base.name, subnet.id);
                subnet
            }
        };
        Ok(subnet_from_storage)
    }

    pub async fn get_subnet(&self, id: &Uuid) -> Result<Option<Subnet>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_by_ids<'a>(&self, ids: &'a [Uuid]) -> Result<Vec<Subnet>> {
        self.storage.get_by_ids(ids).await
    }

    pub async fn get_all_subnets(&self) -> Result<Vec<Subnet>> {
        self.storage.get_all().await
    }

    pub async fn update_subnet(&self, mut subnet: Subnet) -> Result<Subnet> {
        subnet.updated_at = chrono::Utc::now();
        self.storage.update(&subnet).await?;
        tracing::info!("Updated subnet {}: {}", subnet.base.name, subnet.id);
        Ok(subnet)
    }

    pub async fn delete_subnet(&self, id: &Uuid) -> Result<()> {
        let subnet = self
            .get_subnet(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Subnet not found"))?;

        let host_service = self
            .host_service
            .get()
            .ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;

        let hosts = host_service.get_all_hosts().await?;
        let update_futures = hosts.into_iter().filter_map(|mut host| {
            let has_subnet = host
                .base
                .interfaces
                .iter()
                .any(|i| &i.base.subnet_id == id);
            if has_subnet {
                host.base.interfaces = host
                    .base
                    .interfaces
                    .iter()
                    .filter(|i| &i.base.subnet_id != id)
                    .cloned()
                    .collect();
                return Some(host_service.update_host(host));
            }
            None
        });

        try_join_all(update_futures).await?;

        self.storage.delete(id).await?;
        tracing::info!("Deleted subnet {}: {}", subnet.base.name, subnet.id);
        Ok(())
    }
}
