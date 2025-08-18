use serde::{Deserialize, Serialize};
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
    pub domain: Option<Option<String>>,
    pub ip: Option<Option<String>>,
    pub port: Option<Option<u16>>,
    pub path: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub node_type: Option<Option<NodeType>>,
    pub capabilities: Option<Vec<NodeCapability>>,
    pub monitoring_enabled: Option<bool>,
    pub assigned_tests: Option<Vec<AssignedTest>>,
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