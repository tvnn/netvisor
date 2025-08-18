use serde::{Deserialize, Serialize};
use crate::components::{
    tests::types::{TestType, TestResult},
};

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
}