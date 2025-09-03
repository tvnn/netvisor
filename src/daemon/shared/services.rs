use std::sync::Arc;
use anyhow::Result;
use crate::{daemon::{discovery::{service::DaemonDiscoveryService}, runtime::service::DaemonRuntimeService, shared::storage::ConfigStore, subnets::service::DaemonSubnetService}};

pub struct DaemonServiceFactory {
    pub discovery_service: Arc<DaemonDiscoveryService>,
    pub subnet_service: Arc<DaemonSubnetService>,
    pub runtime_service: Arc<DaemonRuntimeService>,
}

impl DaemonServiceFactory {
    pub async fn new(config: Arc<ConfigStore>) -> Result<Self> {
        // Initialize services with proper dependencies

        let discovery_service = Arc::new(DaemonDiscoveryService::new(
            config.clone(),
        ));

        let subnet_service = Arc::new(DaemonSubnetService::new(
            config.clone(),
        ));

        let runtime_service = Arc::new(DaemonRuntimeService::new(
            config.clone(),
        ));
        
        Ok(Self {
            discovery_service,
            subnet_service,
            runtime_service,
        })
    }
}