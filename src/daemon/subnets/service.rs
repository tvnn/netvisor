use anyhow::{Result};
use anyhow::anyhow;
use get_if_addrs::{get_if_addrs, Interface};
use std::{sync::Arc};
use crate::daemon::utils::base::{create_system_utils, PlatformSystemUtils};
use crate::server::subnets::types::base::{Subnet};
use crate::{
    daemon::{shared::storage::ConfigStore}
};

pub struct DaemonSubnetService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub utils: PlatformSystemUtils
}

impl DaemonSubnetService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
            utils: create_system_utils()
        }
    }

    pub async fn scan_subnets(&self) -> Result<Vec<Subnet>> {

        let interfaces = get_if_addrs().map_err(|e| anyhow!("Failed to get network interfaces: {}", e))?;

        tracing::debug!("Found {} network interfaces", interfaces.len());

        let subnets: Vec<Subnet> = interfaces.into_iter()
            .filter(|interface| !should_skip_interface(&interface))
            .filter_map(|interface| Subnet::from_interface(&interface))
            .collect();

        Ok(subnets)
    }

}

pub fn should_skip_interface(interface: &Interface) -> bool {
    // Skip loopback, docker bridges, etc.
    let skip_patterns = ["lo", "lo0", "docker", "br-"];
    skip_patterns.iter().any(|pattern| interface.name.starts_with(pattern)) || interface.is_loopback()
}