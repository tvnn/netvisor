use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::components::tests::types::{TestResult, Test};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestTypeCompatibilityInfo {
    pub test_type: String,
    pub display_name: String,
    pub description: String,
    pub contextual_description: String,
    pub is_assigned: bool,
    pub warning: Option<String>,
    pub is_recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AssignedTest {
    pub test: Test,
    pub criticality: TestCriticality,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TestCriticality {
    Critical,        // Failure = NodeStatus::Failed
    Important,       // Failure = NodeStatus::Degraded  
    Informational,   // Failure = NodeStatus::Healthy (just logged)
}

impl TestCriticality {
    pub fn display_name(&self) -> &'static str {
        match self {
            TestCriticality::Critical => "Critical",
            TestCriticality::Important => "Important",
            TestCriticality::Informational => "Informational",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTestResults {
    pub test_results: Vec<TestResult>,
    pub executed_at: DateTime<Utc>,
    pub node_status: NodeStatus,
    pub duration_ms: u64,
}

// Current health state of a node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum NodeStatus {
    Healthy,    // All checks passing
    Degraded,   // Some checks failing but node functional
    Failed,     // Critical checks failing
    Unknown,    // No recent check data
}

impl NodeStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeStatus::Healthy => "Healthy",
            NodeStatus::Degraded => "Degraded",
            NodeStatus::Failed => "Failed",
            NodeStatus::Unknown => "Unknown",
        }
    }
}