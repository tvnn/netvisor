use anyhow::{Error, Result};
use cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use futures::future::join_all;
use anyhow::anyhow;
use get_if_addrs::{get_if_addrs, Interface};
use unzip3::Unzip3;
use std::{sync::Arc};
use crate::daemon::utils::base::{create_system_utils, PlatformSystemUtils, SystemUtils};
use crate::server::subnets::types::base::{NodeSubnetMembership, Subnet, SubnetBase};
use crate::{
    daemon::{shared::storage::ConfigStore}, server::{
        shared::types::api::ApiResponse
    }
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

    pub async fn scan_and_create_subnets(&self) -> Result<(Vec<Subnet>, Vec<NodeSubnetMembership>)> {

        let interfaces = get_if_addrs().map_err(|e| anyhow!("Failed to get network interfaces: {}", e))?;

        tracing::debug!("Found {} network interfaces", interfaces.len());

        let (subnet_futures, mac_futures, interfaces): (Vec<_>, Vec<_>, Vec<Interface>) = interfaces.into_iter()
            .filter(|interface| !self.should_skip_interface(&interface))
            .filter_map(|interface| {
                if let Ok(cidr) = self.calculate_subnet_from_interface(&interface) {
                    let subnet = Subnet::new(SubnetBase {
                        cidr,
                        name: self.generate_subnet_name(&interface),
                        description: None,
                        dns_resolvers: Vec::new(),
                        gateways: Vec::new()
                    });

                    tracing::debug!("Creating subnet {} for NIC {}", subnet.base.cidr, interface.name);

                    let subnet_future = self.create_discovered_subnet(subnet);
                    let mac_future = self.utils.get_mac_address_for_ip(interface.ip());

                    return Some((subnet_future, mac_future, interface));
                }
                tracing::debug!("Could not determine subnet for NIC {}", interface.name);
                None
            })
            .unzip3();

        let (subnets, mac_addresses) = tokio::join!(
            join_all(subnet_futures),
            join_all(mac_futures)
        );

        let (subnets, node_subnet_membership): 
            (Vec<Subnet>, Vec<NodeSubnetMembership>) = subnets.into_iter()
                .zip(mac_addresses)
                .zip(interfaces)
                .filter_map(|((subnet, mac_address), interface)| {

                    if let Ok(sub) = subnet {
                        let node_subnet_membership = NodeSubnetMembership {
                            subnet_id: sub.id,
                            ip_address: interface.ip(),
                            mac_address: mac_address.unwrap_or(None)
                        };

                        return Some((sub, node_subnet_membership))
                    }
                    return None
                })
                .collect();

        Ok((subnets, node_subnet_membership))
    }

    async fn create_discovered_subnet(&self, subnet: Subnet) -> Result<Subnet> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/subnets", server_target.to_string()))
            .json(&subnet)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovered subnet: HTTP {}", response.status());
        }

        let api_response: ApiResponse<Subnet> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response.error.unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create subnet: {}", error_msg);
        }

        let created_subnet = api_response.data
            .ok_or_else(|| anyhow::anyhow!("No subnet data in successful response"))?;

        Ok(created_subnet)
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

    fn should_skip_interface(&self, interface: &Interface) -> bool {
        // Skip loopback, docker bridges, etc.
        let skip_patterns = ["lo", "lo0", "docker", "br-"];
        skip_patterns.iter().any(|pattern| interface.name.starts_with(pattern)) || interface.is_loopback()
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