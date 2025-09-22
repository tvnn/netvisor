use std::net::Ipv4Addr;

use chrono::{DateTime, Utc};
use cidr::{IpCidr, Ipv4Cidr};
use itertools::Itertools;
use pnet::{ipnetwork::IpNetwork};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;

use crate::server::{hosts::types::base::Host, services::types::base::Service, shared::{constants::{Entity}, types::metadata::{EntityMetadataProvider, TypeMetadataProvider}}};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SubnetSource {
    Manual,
    System,
    Discovery(Uuid)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubnetBase {
    pub cidr: IpCidr,
    pub name: String,  // "Home LAN", "VPN Network", etc.
    pub description: Option<String>,
    pub dns_resolvers: Vec<Uuid>,    // [primary_dns, secondary_dns, fallback_dns]
    pub gateways: Vec<Uuid>,         // [default_gateway, backup_gateway]
    pub reverse_proxies: Vec<Uuid>,
    pub hosts: Vec<Uuid>,
    pub subnet_type: SubnetType,
    pub source: SubnetSource,
}

impl Default for SubnetBase {
    fn default() -> Self {
        Self {
            cidr: IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(127, 0, 0, 1), 24).unwrap()),
            name: "New Subnet".to_string(),
            description: None,
            dns_resolvers: Vec::new(),
            gateways: Vec::new(),
            reverse_proxies: Vec::new(),
            hosts: Vec::new(),
            subnet_type: SubnetType::Unknown,
            source: SubnetSource::Manual
        }
    }
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

                let cidr = IpCidr::V4(Ipv4Cidr::new(network_addr, prefix_len).ok()?);

                return Some(Subnet::new(SubnetBase {
                    cidr,
                    description: None,
                    name: interface_name.clone(),
                    subnet_type,
                    dns_resolvers: Vec::new(),
                    gateways: Vec::new(),
                    reverse_proxies: Vec::new(),
                    hosts: Vec::new(),
                    source: SubnetSource::Discovery(daemon_id),
                }))
            }
        }
    }

    pub fn remove_service_relationships(&mut self, service: &Service) {
        self.base.dns_resolvers = self.base.dns_resolvers.iter().filter(|dns_service_id| **dns_service_id != service.id).cloned().collect();
        self.base.gateways = self.base.gateways.iter().filter(|gateway_service_id| **gateway_service_id != service.id).cloned().collect();
        self.base.reverse_proxies = self.base.reverse_proxies.iter().filter(|proxy_service_id| **proxy_service_id != service.id).cloned().collect();
    }

    pub fn create_service_relationships(&mut self, service: &Service, host: &Host) {
        // Only add service relationships if the service has an interface binding on this subnet
        let has_interface_on_subnet = service.base.interface_bindings.iter()
            .any(|binding_id| {
                host.base.interfaces.iter()
                    .any(|interface| interface.id == *binding_id && interface.base.subnet_id == self.id)
            });
        
        if has_interface_on_subnet {
            if service.base.service_type.is_dns_resolver() { 
                self.base.dns_resolvers.push(service.id) 
            }
            if service.base.service_type.is_gateway() { 
                self.base.gateways.push(service.id) 
            }
            if service.base.service_type.is_reverse_proxy() { 
                self.base.reverse_proxies.push(service.id) 
            }
        }
    }

    pub fn remove_host_relationship(&mut self, host: &Host) {
        self.base.hosts = self.base.hosts.iter().filter(|host_id| **host_id != host.id).cloned().collect();
    }
    
    pub fn create_host_relationship(&mut self, host: &Host)  {
        if host.base.interfaces.iter().map(|i| i.base.subnet_id).contains(&self.id) {
            self.base.hosts.push(host.id)   
        }
    }
}

impl PartialEq for Subnet {
    fn eq(&self, other: &Self) -> bool {
        let cidr_match = &self.base.cidr == &other.base.cidr;
        // let sources_match = match (&self.base.source, &other.base.source) {
        //     (SubnetSource::Discovery(daemon_id), SubnetSource::Discovery(other_daemon_id))  => {
        //         daemon_id == other_daemon_id
        //     },
        //     _ => false
        // };
        cidr_match
    }
}

impl Eq for Subnet {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum SubnetType {
    Lan,
    VpnTunnel, 
    DockerBridge,
    Internet,
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

impl EntityMetadataProvider for SubnetType {
    fn color(&self) -> &'static str {
        match self {
            SubnetType::DockerBridge => "blue",
            SubnetType::Lan => "green",
            SubnetType::VpnTunnel => Entity::Vpn.color(),
            SubnetType::Internet => "gray",
            SubnetType::Unknown => "gray",
        }
    }
    fn icon(&self) -> &'static str {
        Entity::Subnet.icon()
    }
}

impl TypeMetadataProvider for SubnetType {
    fn display_name(&self) -> &'static str {
        match self {
            SubnetType::DockerBridge => "Docker Bridge",
            SubnetType::Lan => "Local Area Network",
            SubnetType::VpnTunnel => "VPN Tunnel",
            SubnetType::Internet => "Internet",
            SubnetType::Unknown => "Unknown",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            SubnetType::DockerBridge => "Docker bridge network",
            SubnetType::Lan => "Local area network",
            SubnetType::VpnTunnel => "VPN tunnel network",
            SubnetType::Internet => "Internet network",
            SubnetType::Unknown => "Unknown network type",
        }
    }
}