use std::net::{IpAddr};

use chrono::{DateTime, Utc};
use cidr::IpCidr;
use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::{capabilities::types::base::CapabilityDiscriminants, nodes::types::base::Node};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubnetBase {
    pub cidr: IpCidr,
    pub name: String,  // "Home LAN", "VPN Network", etc.
    pub description: Option<String>,
    
    // Network services (priority-ordered)
    pub dns_resolvers: Vec<Uuid>,    // [primary_dns, secondary_dns, fallback_dns]
    pub gateways: Vec<Uuid>,         // [default_gateway, backup_gateway]
    // Note: VPN servers are just another type of gateway
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
    
    pub fn update_node_relationships(&mut self, node: &Node)  {
        let node_has_dns_capability = node.has_capability(CapabilityDiscriminants::Dns);

        if node_has_dns_capability { self.base.dns_resolvers.push(node.id) }
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