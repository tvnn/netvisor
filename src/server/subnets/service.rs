use anyhow::Result;
use futures::future::try_join_all;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::server::{hosts::{service::HostService, types::api::HostUpdateRequest}, subnets::{storage::SubnetStorage, types::base::Subnet}
};

pub struct SubnetService {
    storage: Arc<dyn SubnetStorage>,
    host_service: Arc<Mutex<Option<Arc<HostService>>>>
}

impl SubnetService {
    pub fn new(storage: Arc<dyn SubnetStorage>) -> Self {
        Self { 
            storage,
            host_service: Arc::new(Mutex::new(None))
        }
    }
    
    pub fn set_host_service(&self, host_service: Arc<HostService>) {
        *self.host_service.lock().unwrap() = Some(host_service);
    }

    /// Create a new subnet
    pub async fn create_subnet(&self, subnet: Subnet) -> Result<Subnet> {

        let all_subnets = self.storage.get_all().await?;

        let subnet_from_storage = match all_subnets.iter().find(|s| subnet.eq(s)) {
            Some(existing_subnet) => {
                existing_subnet.clone()
            }
            None => {
                self.storage.create(&subnet).await?;
                subnet
            }
        };

        Ok(subnet_from_storage)
    }

    pub async fn get_subnet(&self, id: &Uuid) -> Result<Option<Subnet>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_by_ids(&self, ids: &Vec<Uuid>) -> Result<Vec<Subnet>> {
        self.storage.get_by_ids(ids).await
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

        let host_service = self.host_service
            .lock()
            .unwrap()
            .as_ref()
            .expect("Host service not initialized")
            .clone();

        let hosts = host_service.get_all_hosts().await?;
        let update_futures = hosts
            .iter()
            .filter_map(|n| {
                let has_subnet = n.base.interfaces.iter().find(|i| &i.base.subnet_id == id).is_some();
                if has_subnet {
                    let updates = HostUpdateRequest {
                        name: None,
                        hostname: None,
                        description: None,
                        target: None,
                        open_ports: None,
                        interfaces: Some(n.base.interfaces.iter().filter(|i| &i.base.subnet_id != id).cloned().collect()),
                        services: None,
                        groups: None,
                    };
                    return Some(host_service.update_host(&n.id, updates));
                }
                None
            });

        try_join_all(update_futures).await?;

        self.storage.delete(id).await
    }
}