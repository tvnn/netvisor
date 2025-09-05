// src/server/shared/services.rs
use std::sync::Arc;
use anyhow::Result;
use crate::server::{
    daemons::service::DaemonService, node_groups::service::NodeGroupService, nodes::service::NodeService, shared::types::storage::StorageFactory, subnets::service::SubnetService
};

pub struct ServiceFactory {
    pub node_service: Arc<NodeService>,
    pub node_group_service: Arc<NodeGroupService>,
    pub subnet_service: Arc<SubnetService>,
    pub daemon_service: Arc<DaemonService>,
}

impl ServiceFactory {
    pub async fn new(storage: &StorageFactory) -> Result<Self> {
        // Initialize services with proper dependencies

        let node_service = Arc::new(NodeService::new(
            storage.nodes.clone(),
            storage.node_groups.clone(),
            storage.subnets.clone(),
        ));
        
        let node_group_service = Arc::new(NodeGroupService::new(
            storage.node_groups.clone(),
            node_service.clone(),
        ));

        let subnet_service = Arc::new(SubnetService::new(
            storage.subnets.clone()
        ));

        let daemon_service = Arc::new(DaemonService::new(
            storage.daemons.clone(),
            node_service.clone(),
        ));
        
        Ok(Self {
            node_service,
            node_group_service, 
            subnet_service,
            daemon_service,
        })
    }
}