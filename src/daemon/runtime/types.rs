use std::sync::Arc;

use crate::daemon::{shared::{services::DaemonServiceFactory, storage::ConfigStore}, utils::base::PlatformDaemonUtils};

pub struct DaemonAppState {
    pub config: Arc<ConfigStore>,
    pub services: Arc<DaemonServiceFactory>,
    pub utils: PlatformDaemonUtils
}

impl DaemonAppState {
    pub async fn new(config: Arc<ConfigStore>, utils: PlatformDaemonUtils) -> anyhow::Result<Arc<Self>> {
        config.initialize().await?;
        let services = Arc::new(DaemonServiceFactory::new(config.clone()).await?);
        Ok(Arc::new(Self { config, services, utils}))
    }
}