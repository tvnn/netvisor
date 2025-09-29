use std::net::Ipv4Addr;

use crate::server::services::types::definitions::ServiceDefinitionExt;
use chrono::{DateTime, Utc};
use cidr::{IpCidr, Ipv4Cidr};
use itertools::Itertools;
use pnet::ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use strum_macros::{Display, EnumDiscriminants, EnumIter, IntoStaticStr};
use uuid::Uuid;

use crate::server::{
    hosts::types::base::Host,
    services::types::base::Service,
    shared::{
        constants::Entity,
        types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SubnetSource {
    Manual,
    System,
    Discovery(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubnetBase {
    pub cidr: IpCidr,
    pub name: String, // "Home LAN", "VPN Network", etc.
    pub description: Option<String>,
    pub dns_resolvers: Vec<Uuid>, // [primary_dns, secondary_dns, fallback_dns]
    pub gateways: Vec<Uuid>,      // [default_gateway, backup_gateway]
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
            source: SubnetSource::Manual,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            base,
        }
    }

    pub fn from_discovery(
        interface_name: String,
        ip_network: &IpNetwork,
        daemon_id: Uuid,
    ) -> Option<Self> {
        let subnet_type = SubnetType::from_interface_name(&interface_name);

        match ip_network {
            IpNetwork::V6(_) => None,
            IpNetwork::V4(ipv4_network) => {
                let (network_addr, prefix_len) = match (&subnet_type, ipv4_network.prefix()) {
                    // VPN tunnels with /32 -> expand to /24
                    (SubnetType::VpnTunnel, 32) => {
                        let ip_octets = ipv4_network.ip().octets();
                        let network_addr =
                            std::net::Ipv4Addr::new(ip_octets[0], ip_octets[1], ip_octets[2], 0);
                        (network_addr, 24)
                    }
                    // Skip other /32 single IPs
                    (_, 32) => return None,
                    // Normal case - use the network's actual network address and prefix
                    _ => (ipv4_network.network(), ipv4_network.prefix()),
                };

                let cidr = IpCidr::V4(Ipv4Cidr::new(network_addr, prefix_len).ok()?);

                Some(Subnet::new(SubnetBase {
                    cidr,
                    description: None,
                    name: cidr.to_string(),
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
        self.base.dns_resolvers = self
            .base
            .dns_resolvers
            .iter()
            .filter(|dns_service_id| **dns_service_id != service.id)
            .cloned()
            .collect();
        self.base.gateways = self
            .base
            .gateways
            .iter()
            .filter(|gateway_service_id| **gateway_service_id != service.id)
            .cloned()
            .collect();
        self.base.reverse_proxies = self
            .base
            .reverse_proxies
            .iter()
            .filter(|proxy_service_id| **proxy_service_id != service.id)
            .cloned()
            .collect();
    }

    pub fn create_service_relationships(&mut self, service: &Service, host: &Host) {
        // Only add service relationships if the service has an interface binding on this subnet
        let has_interface_on_subnet = service.base.interface_bindings.iter().any(|binding_id| {
            host.base
                .interfaces
                .iter()
                .any(|interface| interface.id == *binding_id && interface.base.subnet_id == self.id)
        });

        if has_interface_on_subnet {
            if service.base.service_definition.is_dns_resolver() {
                self.base.dns_resolvers.push(service.id)
            }
            if service.base.service_definition.is_gateway() {
                self.base.gateways.push(service.id)
            }
            if service.base.service_definition.is_reverse_proxy() {
                self.base.reverse_proxies.push(service.id)
            }
        }
    }

    pub fn remove_host_relationship(&mut self, host: &Host) {
        self.base.hosts = self
            .base
            .hosts
            .iter()
            .filter(|host_id| **host_id != host.id)
            .cloned()
            .collect();
    }

    pub fn create_host_relationship(&mut self, host: &Host) {
        if host
            .base
            .interfaces
            .iter()
            .map(|i| i.base.subnet_id)
            .contains(&self.id)
        {
            self.base.hosts.push(host.id)
        }
    }
}

impl PartialEq for Subnet {
    fn eq(&self, other: &Self) -> bool {
        self.base.cidr == other.base.cidr
        // let sources_match = match (&self.base.source, &other.base.source) {
        //     (SubnetSource::Discovery(daemon_id), SubnetSource::Discovery(other_daemon_id))  => {
        //         daemon_id == other_daemon_id
        //     },
        //     _ => false
        // };
        // cidr_match
    }
}

impl Hash for Subnet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base.cidr.hash(state);
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Eq,
    PartialEq,
    Hash,
    EnumDiscriminants,
    EnumIter,
    IntoStaticStr,
)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum SubnetType {
    Lan,
    VpnTunnel,
    DockerBridge,
    Internet,
    Unknown,
    None,
}

impl SubnetType {
    pub fn from_interface_name(interface_name: &str) -> Self {
        if Self::match_interface_names(&["docker", "br-"], interface_name) {
            return SubnetType::DockerBridge;
        }

        if Self::match_interface_names(&["tun", "utun", "wg", "tap", "ppp", "vpn"], interface_name)
        {
            return SubnetType::VpnTunnel;
        }

        if Self::match_interface_names(&["eth", "en", "wlan", "wifi", "eno", "enp"], interface_name)
        {
            return SubnetType::Lan;
        }

        SubnetType::Unknown
    }

    fn match_interface_names(patterns: &[&str], interface_name: &str) -> bool {
        let name_lower = interface_name.to_lowercase();
        patterns.iter().any(|pattern| {
            if *pattern == "br-" {
                // Special case for Docker bridges: br- followed by hex chars
                name_lower.starts_with(pattern)
                    && name_lower
                        .get(pattern.len()..)
                        .map(|rest| {
                            !rest.is_empty() && rest.chars().all(|c| c.is_ascii_alphanumeric())
                        })
                        .unwrap_or(false)
            } else {
                // Original logic for other patterns
                name_lower.starts_with(pattern)
                    && name_lower
                        .get(pattern.len()..)
                        .map(|rest| {
                            rest.is_empty() || rest.chars().next().unwrap().is_ascii_digit()
                        })
                        .unwrap_or(false)
            }
        })
    }
}

impl HasId for SubnetType {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for SubnetType {
    fn color(&self) -> &'static str {
        match self {
            SubnetType::DockerBridge => "blue",
            SubnetType::Lan => Entity::Subnet.color(),
            SubnetType::VpnTunnel => Entity::Vpn.color(),
            SubnetType::Internet => "blue",
            SubnetType::Unknown => "gray",
            SubnetType::None => "gray",
        }
    }
    fn icon(&self) -> &'static str {
        match self {
            SubnetType::Internet => "Globe",
            _ => Entity::Subnet.icon(),
        }
    }
}

impl TypeMetadataProvider for SubnetType {
    fn name(&self) -> &'static str {
        match self {
            SubnetType::DockerBridge => "Docker Bridge",
            SubnetType::Lan => "LAN",
            SubnetType::VpnTunnel => "VPN",
            SubnetType::Internet => "Internet",
            SubnetType::Unknown => "Unknown",
            SubnetType::None => "No Subnet",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            SubnetType::DockerBridge => "Docker bridge network",
            SubnetType::Lan => "Local area network",
            SubnetType::VpnTunnel => "Virtual private network",
            SubnetType::Internet => "Internet",
            SubnetType::Unknown => "Unknown network type",
            SubnetType::None => "No Subnet",
        }
    }
}
