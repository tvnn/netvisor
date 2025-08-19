use serde::{Deserialize, Serialize};
use crate::components::nodes::types::base::NodeTarget;
use crate::components::nodes::types::base::DetectedService;
use crate::components::nodes::types::topology::GraphPosition;
use crate::components::nodes::types::tests::NodeStatus;

use super::{
    base::{NodeBase, Node},
    types_capabilities::{NodeType, NodeCapability},
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
    pub capabilities: Option<Vec<NodeCapability>>,
    
    // Monitoring
    pub assigned_tests: Option<Vec<AssignedTest>>,
    pub monitoring_enabled: Option<bool>,
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