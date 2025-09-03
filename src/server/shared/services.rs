// src/server/shared/services.rs
use std::sync::Arc;
use anyhow::Result;
use crate::server::{
    daemons::service::DaemonService, diagnostics::service::DiagnosticService, node_groups::service::NodeGroupService, nodes::service::NodeService, shared::types::storage::StorageFactory, subnets::service::SubnetService, tests::service::TestService
};

pub struct ServiceFactory {
    pub node_service: Arc<NodeService>,
    pub node_group_service: Arc<NodeGroupService>,
    pub diagnostic_service: Arc<DiagnosticService>,
    pub test_service: Arc<TestService>,
    pub subnet_service: Arc<SubnetService>,
    pub daemon_service: Arc<DaemonService>,
}

impl ServiceFactory {
    pub async fn new(storage: &StorageFactory) -> Result<Self> {
        // Initialize services with proper dependencies
        let test_service = Arc::new(TestService::new());

        let node_service = Arc::new(NodeService::new(
            storage.nodes.clone(),
            storage.node_groups.clone(),
            storage.subnets.clone(),
        ));
        
        let node_group_service = Arc::new(NodeGroupService::new(
            storage.node_groups.clone(),
            node_service.clone(),
        ));
        
        let diagnostic_service = Arc::new(DiagnosticService::new(
            storage.diagnostics.clone(),
            node_service.clone(),
            node_group_service.clone(),
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
            diagnostic_service,
            test_service,
            subnet_service,
            daemon_service,
        })
    }
}