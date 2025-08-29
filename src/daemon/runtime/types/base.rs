use std::sync::Arc;

use crate::daemon::{discovery::service::DaemonDiscoveryService, shared::storage::ConfigStore};

pub struct DaemonState {
    pub config: Arc<ConfigStore>,
    pub discovery_service: Arc<DaemonDiscoveryService>
}