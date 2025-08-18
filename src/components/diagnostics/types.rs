use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::components::{nodes::types::tests::{NodeStatus, NodeTestResults}, tests::types::Timer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExecutionBase {
    pub group_id: String,
    pub trigger_reason: String,
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
    pub created_at: DateTime<Utc>,
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
            status: DiagnosticStatus::NotStarted,
            generated_remediation_id: None,
            started_at: timer.datetime(),
            completed_at: None,
            created_at: timer.datetime(),
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
            DiagnosticStatus::Success
        } else {
            DiagnosticStatus::Unknown
        };
    }

    /// Get a summary of the diagnostic results
    pub fn get_summary(&self) -> DiagnosticSummary {
        let total_nodes = self.node_results.len();
        let healthy_nodes = self.node_results.iter().filter(|nr| nr.node_status == NodeStatus::Healthy).count();
        let degraded_nodes = self.node_results.iter().filter(|nr| nr.node_status == NodeStatus::Degraded).count();
        let failed_nodes = self.node_results.iter().filter(|nr| nr.node_status == NodeStatus::Failed).count();

        let total_tests = self.node_results.iter().map(|nr| nr.test_results.len()).sum();
        let passed_tests = self.node_results.iter()
            .flat_map(|nr| &nr.test_results)
            .filter(|tr| tr.success)
            .count();

        DiagnosticSummary {
            total_nodes,
            healthy_nodes,
            degraded_nodes,
            failed_nodes,
            total_tests,
            passed_tests,
            failed_tests: total_tests - passed_tests,
        }
    }
}

/// Status of a diagnostic execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticStatus {
    NotStarted,  // Diagnostic created but run not started
    Success,     // All checks passed
    PartialFail, // Some checks failed, mixed results
    Failed,      // All critical checks failed
    Unknown      // Fallback
}

impl DiagnosticStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            DiagnosticStatus::NotStarted => "Not Started",
            DiagnosticStatus::Success => "Success",
            DiagnosticStatus::PartialFail => "Partial Failure",
            DiagnosticStatus::Failed => "Failed",
            DiagnosticStatus::Unknown => "Unknown",
        }
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
    pub summary: DiagnosticSummary,
}

/// Response for listing diagnostic executions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticListResponse {
    pub executions: Vec<DiagnosticExecution>,
    pub total: usize,
}

/// Query parameters for listing diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticListQuery {
    pub group_id: Option<String>,
    pub status: Option<DiagnosticStatus>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Response for group diagnostic status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupDiagnosticStatusResponse {
    pub group_id: String,
    pub latest_status: Option<DiagnosticStatus>,
    pub latest_execution_id: Option<String>,
    pub last_execution_time: Option<chrono::DateTime<chrono::Utc>>,
    pub total_executions: usize,
}

/// Statistics about diagnostic executions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticStatistics {
    pub total_executions: usize,
    pub successful_executions: usize,
    pub failed_executions: usize,
    pub partial_failure_executions: usize,
    pub average_execution_time_ms: Option<i64>,
}

/// Response for diagnostic statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticStatisticsResponse {
    pub statistics: DiagnosticStatistics
}