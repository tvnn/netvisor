use std::net::{IpAddr, Ipv4Addr};

use anyhow::Error;
use chrono::{DateTime, Utc};
use cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use get_if_addrs::Interface;
use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;

use crate::server::{capabilities::types::base::CapabilityDiscriminants, nodes::types::base::Node};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubnetBase {
    pub cidr: IpCidr,
    pub name: String,  // "Home LAN", "VPN Network", etc.
    pub description: Option<String>,
    pub dns_resolvers: Vec<Uuid>,    // [primary_dns, secondary_dns, fallback_dns]
    pub gateways: Vec<Uuid>,         // [default_gateway, backup_gateway]
    pub subnet_type: SubnetType
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Subnet {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: SubnetBase,
}

impl Subnet {
    pub fn new(base: SubnetBase) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            base
        }
    }

    pub fn from_interface(interface: &Interface) -> Option<Self> {
        if let Ok(calculated_cidr) = Subnet::calculate_cidr_from_interface(interface) {

            let subnet_type = SubnetType::from_interface_name(&interface.name);

            match calculated_cidr {
                IpCidr::V6(_) => return None,
                IpCidr::V4(ipv4_cidr) => {

                    let mut cidr = calculated_cidr;
                    
                    if subnet_type == SubnetType::VpnTunnel && ipv4_cidr.network_length() == 32 {
                        let octets = ipv4_cidr.first().address().octets();
                        cidr = IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(octets[0], octets[1], octets[2], 0), 24).ok()?);
                    }

                    return Some(Subnet::new(SubnetBase {
                        cidr,
                        description: None,
                        name: interface.name.clone(),
                        subnet_type,
                        dns_resolvers: Vec::new(),
                        gateways: Vec::new(),
                    }))
                }
            }
        };
        return None
    }
    
    pub fn update_node_relationships(&mut self, node: &Node)  {
        if node.has_capability(CapabilityDiscriminants::Dns) { self.base.dns_resolvers.push(node.id) }
        if node.is_gateway_for_subnet(&self) { self.base.gateways.push(node.id) }
    }

    fn calculate_cidr_from_interface(interface: &Interface) -> Result<IpCidr, Error> {
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
}

impl PartialEq for Subnet {
    fn eq(&self, other: &Self) -> bool {
        self.base.cidr == other.base.cidr && self.base.gateways[0] == other.base.gateways[0]
    }
}

impl Eq for Subnet {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NodeSubnetMembership {
    pub subnet_id: Uuid,
    pub ip_address: IpAddr,
    pub mac_address: Option<MacAddress>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum SubnetType {
    LocalLan,
    VpnTunnel, 
    DockerBridge,
    Unknown
}

impl SubnetType {
    pub fn from_interface_name(interface_name: &String) -> Self {
        
        if Self::match_interface_names(&["docker", "br-"], interface_name) {
            return SubnetType::DockerBridge;
        }

        if Self::match_interface_names(&["tun", "wg", "tap", "ppp", "vpn"], interface_name){
            return SubnetType::VpnTunnel;
        }

        if Self::match_interface_names(&["eth", "en", "wlan", "wifi", "eno", "enp"], interface_name) {
            return SubnetType::LocalLan;
        }

        SubnetType::Unknown
    }

    fn match_interface_names(patterns: &[&str], interface_name: &String) -> bool {
        let name_lower = interface_name.to_lowercase();
        patterns.iter().any(|pattern| {
            name_lower.starts_with(pattern) && 
            // Ensure it's followed by a digit or end of string (not another letter)
            name_lower.get(pattern.len()..)
                .map(|rest| rest.is_empty() || rest.chars().next().unwrap().is_ascii_digit())
                .unwrap_or(false)
        })
    }
}