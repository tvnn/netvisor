use std::{net::{IpAddr, Ipv4Addr}};

use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoDiscriminant;
use crate::server::{nodes::types::{status::NodeStatus, targets::NodeTarget}, services::types::base::{Service, ServiceDiscriminants}, subnets::types::base::{NodeSubnetMembership, Subnet}};
use super::{
    types::{NodeType},
};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NodeBase {
    pub name: String,
    pub node_type: NodeType,
    pub hostname: Option<String>,
    pub description: Option<String>,
    pub target: NodeTarget,
    pub subnets: Vec<NodeSubnetMembership>,
    
    // Discovery & Service Data
    pub discovery_status: Option<DiscoveryStatus>,
    pub services: Vec<Service>,
    pub dns_resolver_node_id: Option<String>,
    
    // Monitoring
    pub status: NodeStatus,
    pub monitoring_interval: u16,
    pub node_groups: Vec<Uuid>,
}

// Make any changes to NodeBody to NodeUpdate in types/api.rs

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum DiscoveryStatus {
    Discovered { session_id: Uuid, discovered_at: DateTime<Utc> },
    Reviewed,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Node {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub base: NodeBase,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        
        let macs_a: Vec<Option<MacAddress>> = self.base.subnets.iter().map(|s| s.mac_address).collect();
        let macs_b: Vec<Option<MacAddress>> = other.base.subnets.iter().map(|s| s.mac_address).collect();

        let mac_match = macs_a.iter().any(|mac_a| {
            macs_b.iter().any(|mac_b| {
                match (mac_a, mac_b) {
                    (Some(a), Some(b)) => !vec!(
                        MacAddress::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                        MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
                    ).contains(&a) && a == b,
                    (_, _) => false
                }
            })
        });

        let subnet_ip_match = self.base.subnets.iter().any(|subnet_a| {
            other.base.subnets.iter().any(|subnet_b| {
                subnet_a.subnet_id == subnet_b.subnet_id && subnet_a.ip_address == subnet_b.ip_address
            })
        });

        mac_match || subnet_ip_match
    }
}

impl Node {
    pub fn new(base: NodeBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            last_seen: None,
            base,
        }
    }

    pub fn default_subnet(&self) -> &NodeSubnetMembership {
        &self.base.subnets[0]
    }

    // Node group management
    pub fn add_to_group(&mut self, group_id: Uuid) -> Self {
        if !self.base.node_groups.contains(&group_id) {
            self.base.node_groups.push(group_id);
            self.updated_at = chrono::Utc::now();
        }
        self.clone()
    }
    
    pub fn remove_from_group(&mut self, group_id: &Uuid) {
        self.base.node_groups.retain(|id| id != group_id);
        self.updated_at = chrono::Utc::now();
    }

    pub fn has_service(&self, service_discriminants: ServiceDiscriminants) -> bool{
        self.base.services.iter().any(|c| c.discriminant() == service_discriminants)
    }

    pub fn get_service(&self, service_discriminants: ServiceDiscriminants) -> Option<&Service>{
        self.base.services.iter().find(|c| c.discriminant() == service_discriminants)
    }

    pub fn add_service(&mut self, service: Service) {        
        self.base.services.push(service);
    }

    pub fn is_gateway_for_subnet(&self, subnet: &Subnet) -> bool {
        self.base.subnets.iter().any(|subnet_membership| {
            if subnet_membership.subnet_id == subnet.id {
                 let ip_octets = match subnet_membership.ip_address.to_canonical() {
                    IpAddr::V4(ip) => ip.octets(),
                    IpAddr::V6(ip ) => ip.to_ipv4().unwrap_or(Ipv4Addr::LOCALHOST).octets()
                 };

                 return ip_octets.last() == Some(&u8::from(1)) || ip_octets.last() == Some(&u8::from(254))
            }
            return false
        })
    }
}