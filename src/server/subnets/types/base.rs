use std::net::{IpAddr};
use chrono::{DateTime, Utc};
use cidr::{IpCidr, Ipv4Cidr};
use mac_address::MacAddress;
use pnet::{ipnetwork::IpNetwork};
use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;

use crate::server::{services::types::base::{ServiceCategory}, nodes::types::base::Node};

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

    pub fn from_interface(interface_name: &String, ip_network: &IpNetwork) -> Option<Self> {

        let subnet_type = SubnetType::from_interface_name(&interface_name);

        match ip_network {
            IpNetwork::V6(_) => return None,
            IpNetwork::V4(ipv4_network) => {
                
                let (network_addr, prefix_len) = if subnet_type == SubnetType::VpnTunnel && ipv4_network.prefix() == 32 {
                    // For VPN tunnels with /32, assume /24 network from the interface IP
                    let ip_octets = ipv4_network.ip().octets();
                    let network_addr = std::net::Ipv4Addr::new(ip_octets[0], ip_octets[1], ip_octets[2], 0);
                    (network_addr, 24)
                } 
                else { (ipv4_network.network(), ipv4_network.prefix()) };

                return Some(Subnet::new(SubnetBase {
                    cidr: IpCidr::V4(Ipv4Cidr::new(network_addr, prefix_len).ok()?),
                    description: None,
                    name: interface_name.clone(),
                    subnet_type,
                    dns_resolvers: Vec::new(),
                    gateways: Vec::new(),
                }))
            }
        }
    }
    
    pub fn update_node_relationships(&mut self, node: &Node)  {
        if node.base.services.iter().any(|c| [ServiceCategory::DNS, ServiceCategory::AdBlock].contains(&c.discriminant().service_category())) { self.base.dns_resolvers.push(node.id) }
        if node.is_gateway_for_subnet(&self) { self.base.gateways.push(node.id) }
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

        if Self::match_interface_names(&["tun", "utun", "wg", "tap", "ppp", "vpn"], interface_name){
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