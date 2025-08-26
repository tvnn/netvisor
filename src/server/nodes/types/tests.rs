use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::{nodes::types::{criticality::TestCriticality, status::NodeStatus}, tests::types::{base::Test, execution::TestResult}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestTypeCompatibilityInfo {
    pub test_type: String,
    pub display_name: String,
    pub description: String,
    pub is_assigned: bool,
    pub warning: Option<String>,
    pub is_recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AssignedTest {
    pub test: Test,
    pub criticality: TestCriticality,
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
    pub node_id: Uuid,
    pub executed_at: DateTime<Utc>,
    pub node_status: NodeStatus,
    pub duration_ms: u64,
}