use anyhow::{Result};
use std::{sync::Arc};
use crate::daemon::utils::base::{create_system_utils, PlatformSystemUtils, SystemUtils};
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

        let interfaces = self.utils.get_own_interfaces();

        tracing::debug!("Found {} network interfaces", interfaces.len());

        let subnets: Vec<Subnet> = interfaces.into_iter()
            .filter(|interface| !interface.is_loopback())
            .flat_map(|interface| {
                interface.ips.iter().filter_map(|ip| Subnet::from_interface(&interface.name, &ip))
                .collect::<Vec<Subnet>>()
            })
            .collect();

        Ok(subnets)
    }

}