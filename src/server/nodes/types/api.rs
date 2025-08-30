use cidr::IpCidr;
use serde::{Deserialize, Serialize};
use crate::server::capabilities::types::base::Capability;
use crate::server::nodes::types::base::DiscoveryStatus;
use crate::server::nodes::{types::status::NodeStatus};
use crate::server::nodes::types::targets::NodeTarget;

use super::{
    base::{NodeBase, Node},
    types::{NodeType},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeRequest {
    #[serde(flatten)]
    pub node: NodeBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeRequest {
    pub name: Option<String>,
    pub node_type: Option<NodeType>,
    pub target: Option<NodeTarget>,
    pub description: Option<Option<String>>,
    pub subnets: Option<Vec<IpCidr>>,
        
    // Discovery & Capability Data
    pub discovery_status: Option<DiscoveryStatus>,
    pub capabilities: Option<Vec<Capability>>,
    pub dns_resolver_node_id: Option<String>,
    
    // Monitoring
    pub status: Option<NodeStatus>,
    pub monitoring_interval: Option<u16>,
    pub node_groups: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub node: Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListResponse {
    pub nodes: Vec<Node>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct CompatibilityResponse<T> {
    pub recommendations: Option<Vec<T>>,
    pub warnings: Option<Vec<T>>,
}