use serde::{Deserialize, Serialize};
use crate::components::nodes::{capabilities::base::NodeCapability, types::status::NodeStatus};
use crate::components::nodes::types::targets::NodeTarget;
use crate::components::nodes::types::base::DetectedService;
use crate::components::nodes::capabilities::base::{deserialize_optional_capabilities_from_discriminants, serialize_optional_capabilities_as_discriminants};
use crate::components::nodes::types::topology::GraphPosition;

use super::{
    base::{NodeBase, Node},
    types::{NodeType},
    tests::{AssignedTest}
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
        
    // Discovery & Capability Data
    pub open_ports: Option<Vec<u16>>,
    pub detected_services: Option<Vec<DetectedService>>,
    pub mac_address: Option<Option<String>>,
    #[serde(
        serialize_with = "serialize_optional_capabilities_as_discriminants",
        deserialize_with = "deserialize_optional_capabilities_from_discriminants"
    )]
    pub capabilities: Option<Vec<NodeCapability>>,
    
    // Monitoring
    pub assigned_tests: Option<Vec<AssignedTest>>,
    pub monitoring_interval: Option<u16>,
    pub node_groups: Option<Vec<String>>,
    
    // Topology visualization
    pub position: Option<Option<GraphPosition>>,
    pub current_status: Option<NodeStatus>,
    pub subnet_membership: Option<Vec<String>>,
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