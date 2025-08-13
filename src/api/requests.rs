use serde::{Deserialize, Serialize};
use crate::core::{NodeType, NodeCapability, TestType, TestConfiguration, TestCriticality};

// Node-related requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeRequest {
    pub name: String,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub node_type: Option<NodeType>,
    pub capabilities: Option<Vec<NodeCapability>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub node_type: Option<NodeType>,
    pub capabilities: Option<Vec<NodeCapability>>,
    pub monitoring_enabled: Option<bool>,
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
    pub name: String,
    pub description: String,
    pub node_sequence: Vec<String>,
    pub auto_diagnostic_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub node_sequence: Option<Vec<String>>,
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