use serde::{Deserialize, Serialize};
use crate::core::{NodeBase, NodeGroupBase, NodeType, NodeCapability, AssignedTest, TestType, TestConfiguration, TestCriticality};

// Node-related requests
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
pub struct AssignTestRequest {
    pub node_id: String,
    pub test_type: TestType,
    pub test_config: TestConfiguration,
    pub criticality: TestCriticality,
    pub monitor_interval_minutes: Option<u32>,
}

// Node Group requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeGroupRequest {
    #[serde(flatten)]
    pub group: NodeGroupBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub node_sequence: Option<Vec<String>>,  // Ordered diagnostic sequence
    pub auto_diagnostic_enabled: Option<bool>,
}

// Diagnostic requests
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteDiagnosticRequest {
    pub group_id: String,
    pub trigger_reason: Option<String>,
}

// Ad-hoc test requests
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteAdhocTestRequest {
    pub node_id: String,
    pub test_type: TestType,
    pub test_config: TestConfiguration,
}

// Discovery requests
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverNetworkRequest {
    pub subnet: Option<String>, // CIDR notation like "192.168.1.0/24"
    pub port_scan: Option<bool>,
    pub include_known_services: Option<bool>,
}

// Monitoring requests
#[derive(Debug, Serialize, Deserialize)]
pub struct SetMonitoringRequest {
    pub node_id: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMonitoringIntervalRequest {
    pub node_id: String,
    pub test_type: TestType,
    pub interval_minutes: Option<u32>, // None = disable monitoring for this test
}