use anyhow::{Error, Result};
use cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use anyhow::anyhow;
use get_if_addrs::{get_if_addrs, Interface};
use std::{sync::Arc};
use crate::daemon::utils::base::{create_system_utils, PlatformSystemUtils};
use crate::server::subnets::types::base::{Subnet, SubnetBase};
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
            .filter_map(|interface| {
                if let Ok(cidr) = self.calculate_subnet_from_interface(&interface) {

                    if cidr.is_ipv6() {
                        return None
                    }

                    let subnet = Subnet::new(SubnetBase {
                        cidr,
                        name: self.generate_subnet_name(&interface),
                        description: None,
                        dns_resolvers: Vec::new(),
                        gateways: Vec::new()
                    });

                    tracing::debug!("Creating subnet {} for NIC {}", subnet.base.cidr, interface.name);

                    return Some(subnet);
                }
                tracing::debug!("Could not determine subnet for NIC {}", interface.name);
                None
            })
            .collect();

        Ok(subnets)
    }

    fn calculate_subnet_from_interface(&self, interface: &Interface) -> Result<IpCidr, Error> {
        match &interface.addr {
            get_if_addrs::IfAddr::V4(v4_addr) => {
                let netmask = v4_addr.netmask;
                let prefix_len = netmask.octets().iter()
                    .map(|&octet| octet.count_ones())
                    .sum::<u32>() as u8;

                let network = std::net::Ipv4Addr::from(
                    u32::from(v4_addr.ip) & u32::from(netmask)
                );

                return Ok(IpCidr::V4(Ipv4Cidr::new(network, prefix_len)?));
            }
            get_if_addrs::IfAddr::V6(v6_addr) => {
                let netmask = v6_addr.netmask;
                let prefix_len = netmask.octets().iter()
                    .map(|&octet| octet.count_ones())
                    .sum::<u32>() as u8;

                let ip_bytes = v6_addr.ip.octets();
                let mask_bytes = netmask.octets();
                let mut network_bytes = [0u8; 16];
                for i in 0..16 {
                    network_bytes[i] = ip_bytes[i] & mask_bytes[i];
                }
                let network = std::net::Ipv6Addr::from(network_bytes);

                return Ok(IpCidr::V6(Ipv6Cidr::new(network, prefix_len)?));
            }
        }
    }

    fn generate_subnet_name(&self, interface: &Interface) -> String {
        if self.is_vpn_interface(&interface.name) {
            format!("VPN Network ({})", interface.name)
        } else {
            format!("LAN ({})", interface.name) 
        }
    }

    fn is_vpn_interface(&self, name: &str) -> bool {
        ["tun", "wg", "tap", "ppp", "vpn"]
            .iter()
            .any(|pattern| name.to_lowercase().starts_with(pattern))
    }
}

pub fn should_skip_interface(interface: &Interface) -> bool {
    // Skip loopback, docker bridges, etc.
    let skip_patterns = ["lo", "lo0", "docker", "br-"];
    skip_patterns.iter().any(|pattern| interface.name.starts_with(pattern)) || interface.is_loopback()
}