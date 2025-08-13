use serde::{Deserialize, Serialize};
use crate::core::{Node, NodeGroup, DiagnosticExecution, TestResult, NodeType, TestType};

// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// Node responses
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub node: Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListResponse {
    pub nodes: Vec<Node>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeCompatibilityResponse {
    pub node_id: String,
    pub compatible_test_types: Vec<TestType>,
    pub incompatible_test_types: Vec<TestType>,
    pub missing_capabilities: Vec<String>,
}

// Node Group responses
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeGroupResponse {
    pub group: NodeGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeGroupListResponse {
    pub groups: Vec<NodeGroup>,
    pub total: usize,
}

// Diagnostic responses
#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticExecutionResponse {
    pub execution: DiagnosticExecution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticHistoryResponse {
    pub executions: Vec<DiagnosticExecution>,
    pub total: usize,
}

// Test execution responses
#[derive(Debug, Serialize, Deserialize)]
pub struct TestResultResponse {
    pub result: TestResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTestResultsResponse {
    pub node_id: String,
    pub results: Vec<TestResult>,
    pub executed_at: String,
}

// Discovery responses
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub ip: String,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub open_ports: Vec<u16>,
    pub suggested_node_type: NodeType,
    pub detected_services: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkDiscoveryResponse {
    pub discovered_devices: Vec<DiscoveredDevice>,
    pub scan_duration_ms: u64,
    pub subnet_scanned: String,
}

// Monitoring responses
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringStatusResponse {
    pub node_id: String,
    pub enabled: bool,
    pub active_tests: Vec<MonitoringTestStatus>,
    pub last_check: Option<String>,
    pub next_check: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringTestStatus {
    pub test_type: TestType,
    pub enabled: bool,
    pub interval_minutes: Option<u32>,
    pub last_result: Option<TestResult>,
    pub next_execution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringOverviewResponse {
    pub total_nodes: usize,
    pub monitoring_enabled: usize,
    pub healthy_nodes: usize,
    pub degraded_nodes: usize,
    pub failed_nodes: usize,
    pub unknown_nodes: usize,
    pub recent_failures: Vec<RecentFailure>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentFailure {
    pub node_id: String,
    pub node_name: String,
    pub test_type: TestType,
    pub failed_at: String,
    pub error_message: String,
}

// System status responses
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    pub server_status: String,
    pub database_status: String,
    pub monitoring_active: bool,
    pub total_nodes: usize,
    pub total_groups: usize,
    pub uptime_seconds: u64,
}