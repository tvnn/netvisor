use crate::daemon::{
    discovery::service::base::DaemonDiscoveryService, runtime::service::DaemonRuntimeService,
    shared::storage::ConfigStore,
};
use anyhow::Result;
use std::sync::Arc;

pub struct DaemonServiceFactory {
    pub discovery_service: Arc<DaemonDiscoveryService>,
    pub runtime_service: Arc<DaemonRuntimeService>,
}

impl DaemonServiceFactory {
    pub async fn new(config: Arc<ConfigStore>) -> Result<Self> {
        // Initialize services with proper dependencies

        let discovery_service = Arc::new(DaemonDiscoveryService::new(config.clone()));

        let runtime_service = Arc::new(DaemonRuntimeService::new(config.clone()));

        Ok(Self {
            discovery_service,
            runtime_service,
        })
    }
}
