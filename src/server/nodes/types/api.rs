use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::{services::types::base::{Service}, nodes::types::{base::{Node}, targets::NodeTarget}, subnets::types::base::{NodeSubnetMembership, Subnet}};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct NodeUpdateRequest {
    pub name: Option<String>,
    pub hostname: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub target: Option<NodeTarget>,
    pub subnets: Option<Vec<NodeSubnetMembership>>,
    pub services: Option<Vec<Service>>,
    pub node_groups: Option<Vec<Uuid>>,
}

impl NodeUpdateRequest {
    pub fn from_node_group_change(node_groups: Vec<Uuid>) -> Self {
        Self {
            name: None,
            hostname: None,
            description: None,
            target: None,
            subnets: None,
            services: None,
            node_groups: Some(node_groups),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct UpdateNodeResponse {
    pub node: Node,
    pub subnet_changes: NodeSubnetRelationshipChange
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NodeSubnetRelationshipChange {
    pub new_gateway: Vec<Subnet>,
    pub no_longer_gateway: Vec<Subnet>,
    pub new_dns_resolver: Vec<Subnet>,
    pub no_longer_dns_resolver: Vec<Subnet>
}