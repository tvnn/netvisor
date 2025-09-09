use chrono::{DateTime, Utc};
use cidr::{IpCidr, Ipv4Cidr};
use pnet::{ipnetwork::IpNetwork};
use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;

use crate::server::{hosts::types::base::Host, shared::types::metadata::TypeMetadataProvider};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SubnetSource {
    Manual,
    Discovery(Uuid)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubnetBase {
    pub cidr: IpCidr,
    pub name: String,  // "Home LAN", "VPN Network", etc.
    pub description: Option<String>,
    pub dns_resolvers: Vec<Uuid>,    // [primary_dns, secondary_dns, fallback_dns]
    pub gateways: Vec<Uuid>,         // [default_gateway, backup_gateway]
    pub hosts: Vec<Uuid>,
    pub subnet_type: SubnetType,
    pub source: SubnetSource
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

    pub fn from_discovery(interface_name: &String, ip_network: &IpNetwork, daemon_id: Uuid) -> Option<Self> {

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
                    hosts: Vec::new(),
                    source: SubnetSource::Discovery(daemon_id)
                }))
            }
        }
    }
    
    pub fn update_host_relationships(&mut self, host: &Host)  {
        if host.base.services.iter().any(|c| c.discriminant().can_be_dns_resolver()) { self.base.dns_resolvers.push(host.id) }
        if host.base.services.iter().any(|c| c.discriminant().can_be_gateway()) { self.base.gateways.push(host.id) }
        self.base.hosts.push(host.id)
    }
}

impl PartialEq for Subnet {
    fn eq(&self, other: &Self) -> bool {
        let cidr_match = &self.base.cidr == &other.base.cidr;
        let sources_match = match (&self.base.source, &other.base.source) {
            (SubnetSource::Discovery(daemon_id), SubnetSource::Discovery(other_daemon_id))  => {
                daemon_id == other_daemon_id
            },
            _ => false
        };
        cidr_match && sources_match
    }
}

impl Eq for Subnet {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum SubnetType {
    Lan,
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
            return SubnetType::Lan;
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

impl TypeMetadataProvider for SubnetType {
    fn id(&self) -> String {
        self.discriminant().to_string()
    }

    fn display_name(&self) -> &str {
        match self {
            SubnetType::DockerBridge => "Docker Bridge",
            SubnetType::Lan => "Local Area Network",
            SubnetType::VpnTunnel => "VPN Tunnel",
            SubnetType::Unknown => "Unknown",
        }
    }

    fn description(&self) -> &str {
        match self {
            SubnetType::DockerBridge => "Docker bridge network",
            SubnetType::Lan => "Local area network",
            SubnetType::VpnTunnel => "VPN tunnel network",
            SubnetType::Unknown => "Unknown network type",
        }
    }

    fn category(&self) -> &str {
        "subnet"
    }

    fn icon(&self) -> &str {
        "Network"
    }

    fn color(&self) -> &str {
        match self {
            SubnetType::DockerBridge => "blue",
            SubnetType::Lan => "green",
            SubnetType::VpnTunnel => "purple",
            SubnetType::Unknown => "gray",
        }
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}