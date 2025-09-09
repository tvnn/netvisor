use anyhow::Result;
use futures::future::try_join_all;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::server::{nodes::{service::NodeService, types::api::NodeUpdateRequest}, subnets::{storage::SubnetStorage, types::base::Subnet}
};

pub struct SubnetService {
    storage: Arc<dyn SubnetStorage>,
    node_service: Arc<Mutex<Option<Arc<NodeService>>>>
}

impl SubnetService {
    pub fn new(storage: Arc<dyn SubnetStorage>) -> Self {
        Self { 
            storage,
            node_service: Arc::new(Mutex::new(None))
        }
    }
    
    pub fn set_node_service(&self, node_service: Arc<NodeService>) {
        *self.node_service.lock().unwrap() = Some(node_service);
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

    pub async fn get_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Subnet>> {
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

        let node_service = self.node_service
            .lock()
            .unwrap()
            .as_ref()
            .expect("Node service not initialized")
            .clone();

        let nodes = node_service.get_all_nodes().await?;
        let update_futures = nodes
            .iter()
            .filter_map(|n| {
                let has_subnet = n.base.subnets.iter().find(|s| &s.subnet_id == id).is_some();
                if has_subnet {
                    let updates = NodeUpdateRequest {
                        name: None,
                        hostname: None,
                        description: None,
                        target: None,
                        open_ports: None,
                        subnets: Some(n.base.subnets.iter().filter(|s| &s.subnet_id != id).cloned().collect()),
                        services: None,
                        node_groups: None,
                    };
                    return Some(node_service.update_node(&n.id, updates));
                }
                None
            });

        try_join_all(update_futures).await?;

        self.storage.delete(id).await
    }
}