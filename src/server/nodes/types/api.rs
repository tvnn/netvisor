use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::{capabilities::types::base::{Capability}, nodes::types::{base::{DiscoveryStatus, Node}, status::NodeStatus, targets::NodeTarget, types::NodeType}, subnets::types::base::{NodeSubnetMembership, Subnet}};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct NodeUpdateRequest {
    pub name: Option<String>,
    pub node_type: Option<NodeType>,
    pub hostname: Option<Option<String>>,
    pub description: Option<Option<String>>,

    pub target: Option<NodeTarget>,
    pub subnets: Option<Vec<NodeSubnetMembership>>,
    
    // Discovery & Capability Data
    pub discovery_status: Option<Option<DiscoveryStatus>>,
    pub capabilities: Option<Vec<Capability>>,
    pub dns_resolver_node_id: Option<Option<String>>,
    
    // Monitoring
    pub status: Option<NodeStatus>,
    pub monitoring_interval: Option<u16>,
    pub node_groups: Option<Vec<Uuid>>,
}

impl NodeUpdateRequest {
    pub fn from_node_group_change(node_groups: Vec<Uuid>) -> Self {
        Self {
            name: None,
            node_type: None,
            hostname: None,
            description: None,
            target: None,
            subnets: None,
            discovery_status: None,
            capabilities: None,
            dns_resolver_node_id: None,
            status: None,
            monitoring_interval: None,
            node_groups: Some(node_groups),
        }
    }
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct UpdateNodeResponse {
    pub node: Node,
    pub subnet_changes: NodeSubnetRelationshipChange
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
pub struct NodeSubnetRelationshipChange {
    pub new_gateway: Vec<Subnet>,
    pub no_longer_gateway: Vec<Subnet>,
    pub new_dns_resolver: Vec<Subnet>,
    pub no_longer_dns_resolver: Vec<Subnet>
}