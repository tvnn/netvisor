use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use crate::{components::{nodes::types::{status::NodeStatus, tests::NodeTestResults}, tests::types::execution::Timer}, shared::metadata::TypeMetadataProvider};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticTrigger {
    Scheduled,
    NodeFailure(String), // Node ID that failed
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExecutionBase {
    pub group_id: String,
    pub trigger_reason: DiagnosticTrigger,
}

/// Represents the execution of diagnostics on a node group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExecution {
    pub id: String,
    pub node_results: Vec<NodeTestResults>,
    pub status: DiagnosticStatus,
    pub generated_remediation_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub base: DiagnosticExecutionBase,
}

impl DiagnosticExecution {
    pub fn new(base: DiagnosticExecutionBase) -> Self {
        let timer = Timer::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            base,
            node_results: Vec::new(),
            status: DiagnosticStatus::Started,
            generated_remediation_id: None,
            started_at: timer.datetime(),
            completed_at: None,
        }
    }

    /// Add results for a node and update overall status
    pub fn add_node_results(&mut self, node_results: NodeTestResults) {
        self.node_results.push(node_results);
        self.update_status();
    }

    /// Complete the diagnostic execution
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.update_status();
    }

    /// Update overall status based on node results
    fn update_status(&mut self) {
        if self.node_results.is_empty() {
            self.status = DiagnosticStatus::Failed; // No results = failed
            return;
        }

        let has_failed = self.node_results.iter().any(|nr| nr.node_status == NodeStatus::Failed);
        let has_degraded = self.node_results.iter().any(|nr| nr.node_status == NodeStatus::Degraded);
        let all_healthy = self.node_results.iter().all(|nr| nr.node_status == NodeStatus::Healthy);

        self.status = if has_failed {
            DiagnosticStatus::Failed
        } else if has_degraded {
            DiagnosticStatus::PartialFail
        } else if all_healthy {
            DiagnosticStatus::Completed
        } else {
            DiagnosticStatus::Unknown
        };
    }
}

/// Status of a diagnostic execution
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Serialize, Deserialize))]
pub enum DiagnosticStatus {
    Started,
    Completed,   
    PartialFail, 
    Failed,      
    Unknown      
}

impl TypeMetadataProvider for DiagnosticStatus {
    fn id(&self) -> String { 
        self.discriminant().to_string()
    }

    fn display_name(&self) -> &str {
        match self {
            DiagnosticStatus::Started => "Started",
            DiagnosticStatus::Completed => "Completed",
            DiagnosticStatus::PartialFail => "Partial Failure",
            DiagnosticStatus::Failed => "Failed",
            DiagnosticStatus::Unknown => "Unknown"
        }
    }

    fn description(&self) -> &str {
        match self {
            DiagnosticStatus::Started => "Diagnostic run started",
            DiagnosticStatus::Completed => "All tests passed",
            DiagnosticStatus::PartialFail => "One or more Important tests failed",
            DiagnosticStatus::Failed => "One or more Critical tests failed",
            DiagnosticStatus::Unknown => "Unknown"
        }
    }

    fn category(&self) -> &str {
        ""
    }

    fn icon(&self) -> &str {
        match self {
            DiagnosticStatus::Started => "Loader2",
            DiagnosticStatus::Completed => "CheckCircle",
            DiagnosticStatus::PartialFail => "AlertTriangle",
            DiagnosticStatus::Failed => "CircleX",
            DiagnosticStatus::Unknown => "CircleQuestionMark"
        }
    }

    fn color(&self) -> &str {
        match self {
            DiagnosticStatus::Started => "blue",
            DiagnosticStatus::Completed => "green",
            DiagnosticStatus::PartialFail => "yellow",
            DiagnosticStatus::Failed => "red",
            DiagnosticStatus::Unknown => "gray"
        }
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}

/// Summary statistics for a diagnostic execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSummary {
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub degraded_nodes: usize,
    pub failed_nodes: usize,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
}

/// Request to execute diagnostics on a node group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteDiagnosticRequest {
    #[serde(flatten)]
    pub diagnostic: DiagnosticExecutionBase,
}

/// Response for diagnostic execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExecutionResponse {
    pub execution: DiagnosticExecution,
}

/// Response for listing diagnostic executions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticListResponse {
    pub executions: Vec<DiagnosticExecution>,
    pub total: usize,
}