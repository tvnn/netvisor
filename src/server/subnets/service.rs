use anyhow::Result;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{subnets::{storage::SubnetStorage, types::base::Subnet}
};

pub struct SubnetService {
    storage: Arc<dyn SubnetStorage>,
}

impl SubnetService {
    pub fn new(storage: Arc<dyn SubnetStorage>) -> Self {
        Self { 
            storage,
        }
    }

    /// Create a new subnet
    pub async fn create_subnet(&self, subnet: Subnet) -> Result<Subnet> {
        self.storage.create(&subnet).await
    }

    pub async fn get_subnet(&self, id: &Uuid) -> Result<Option<Subnet>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_subnets(&self) -> Result<Vec<Subnet>> {
        self.storage.get_all().await
    }

    pub async fn update_subnet(&self, mut subnet: Subnet) -> Result<Subnet> {
        subnet.updated_at = chrono::Utc::now();
        self.storage.update(&subnet).await?;
        Ok(subnet)
    }

    pub async fn delete_subnet(&self, id: &Uuid) -> Result<()> {
        self.storage.delete(id).await
    }
}