use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::components::nodes::types::{NodeTestResults};
use crate::components::tests::types::{TestType, TestConfiguration};

// API Requests and Responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteDiagnosticRequest {
    pub group_id: String,
    pub trigger_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticExecutionResponse {
    pub execution: DiagnosticExecution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticHistoryResponse {
    pub executions: Vec<DiagnosticExecution>,
    pub total: usize,
}

// Diagnostic execution and results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExecution {
    pub id: String,
    pub group_id: String,
    pub group_name: String,
    pub trigger_reason: String,  // Generated contextually
    pub node_results: Vec<NodeTestResults>,
    pub overall_status: DiagnosticStatus,
    pub generated_remediation_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Overall status of a diagnostic execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticStatus {
    Success,     // All checks passed
    PartialFail, // Some checks failed, mixed results
    Failed,      // All critical checks failed  
    InProgress,  // Diagnostic still running
}

// Remediation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remediation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<RemediationStep>,
    pub generated_from_diagnostic: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationStep {
    UserAction {
        description: String,
        instructions: String,
        verification_prompt: String,
    },
    ServerAction {
        description: String,
        target_node_id: String,        // Which node to run test against
        test_type: TestType,           // Now uses enum instead of string
        test_config: TestConfiguration, // Type-safe configuration
    },
}