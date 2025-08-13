use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::node_types::{NodeType, NodeCapability};
use crate::core::test_types::{TestType, TestConfiguration, TestCriticality};

// Node Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    // Existing fields
    pub id: String,
    pub name: String,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    
    // Node type system
    pub node_type: Option<NodeType>,
    pub capabilities: Vec<NodeCapability>,
    
    // Monitoring
    pub assigned_tests: Vec<AssignedTest>,
    pub monitoring_enabled: bool,
    pub node_groups: Vec<String>,
    
    // Topology visualization
    pub position: Option<GraphPosition>,
    pub current_status: NodeStatus,
    pub subnet_membership: Vec<String>,
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedTest {
    pub test_type: TestType,
    pub test_config: TestConfiguration,
    pub monitor_interval_minutes: Option<u32>,  // None = diagnostic-only
    pub enabled: bool,
    pub criticality: TestCriticality,
}

// Graph positioning for topology visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPosition {
    pub x: f32,
    pub y: f32,
    pub z: Option<f32>, // For 3D layouts if needed
}

// Current health state of a node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Healthy,    // All checks passing
    Degraded,   // Some checks failing but node functional
    Failed,     // Critical checks failing
    Unknown,    // No recent check data
}

// Network subnet grouping for topology organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetGroup {
    pub id: String,
    pub name: String,
    pub cidr: String,           // "192.168.1.0/24"
    pub node_ids: Vec<String>,  // Nodes in this subnet
    pub vlan_id: Option<u16>,   // VLAN identifier if applicable
}

// Node Group and Network Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub node_sequence: Vec<String>,  // Ordered diagnostic sequence
    pub auto_diagnostic_enabled: bool,
}

// Network topology generated from node groups and subnet membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    pub id: String,
    pub name: String,
    pub node_groups: Vec<NodeGroup>,     // Logical network paths
    pub subnets: Vec<SubnetGroup>,       // Network clustering
    pub last_updated: DateTime<Utc>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTestResults {
    pub node_id: String,
    pub test_results: Vec<TestResult>,
    pub node_status: NodeStatus,  // Computed from test results + criticality
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_type: TestType,
    pub success: bool,
    pub message: String,
    pub duration_ms: u64,
    pub executed_at: DateTime<Utc>,
    pub details: Option<serde_json::Value>, // Test-specific result data
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

impl NodeStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeStatus::Healthy => "Healthy",
            NodeStatus::Degraded => "Degraded",
            NodeStatus::Failed => "Failed",
            NodeStatus::Unknown => "Unknown",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            NodeStatus::Healthy => "green",
            NodeStatus::Degraded => "yellow",
            NodeStatus::Failed => "red",
            NodeStatus::Unknown => "gray",
        }
    }
}

impl DiagnosticStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            DiagnosticStatus::Success => "Success",
            DiagnosticStatus::PartialFail => "Partial Failure",
            DiagnosticStatus::Failed => "Failed",
            DiagnosticStatus::InProgress => "In Progress",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            DiagnosticStatus::Success => "green",
            DiagnosticStatus::PartialFail => "yellow",
            DiagnosticStatus::Failed => "red",
            DiagnosticStatus::InProgress => "blue",
        }
    }
}

impl Node {
    /// Create a new network node with defaults
    pub fn new(name: String) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            domain: None,
            ip: None,
            port: None,
            path: None,
            description: None,
            created_at: now.clone(),
            updated_at: now,
            node_type: None,
            capabilities: Vec::new(),
            assigned_tests: Vec::new(),
            monitoring_enabled: false,
            node_groups: Vec::new(),
            position: None,
            current_status: NodeStatus::Unknown,
            subnet_membership: Vec::new(),
            last_seen: None,
        }
    }

    /// Update the node's status based on test results
    pub fn compute_status_from_tests(&mut self, test_results: &[TestResult]) {
        if test_results.is_empty() {
            self.current_status = NodeStatus::Unknown;
            return;
        }

        let mut has_critical_failure = false;
        let mut has_important_failure = false;

        // Check each assigned test against results
        for assigned_test in &self.assigned_tests {
            if !assigned_test.enabled {
                continue;
            }

            // Find matching test result
            let test_result = test_results.iter()
                .find(|r| r.test_type == assigned_test.test_type);

            if let Some(result) = test_result {
                if !result.success {
                    match assigned_test.criticality {
                        TestCriticality::Critical => has_critical_failure = true,
                        TestCriticality::Important => has_important_failure = true,
                        TestCriticality::Informational => {
                            // Informational failures don't affect status
                        }
                    }
                }
            } else {
                // No result for this test - treat as failure based on criticality
                match assigned_test.criticality {
                    TestCriticality::Critical => has_critical_failure = true,
                    TestCriticality::Important => has_important_failure = true,
                    TestCriticality::Informational => {}
                }
            }
        }

        // Determine overall status
        self.current_status = if has_critical_failure {
            NodeStatus::Failed
        } else if has_important_failure {
            NodeStatus::Degraded
        } else {
            NodeStatus::Healthy
        };
    }

    /// Get all tests assigned to this node with a specific criticality
    pub fn get_tests_by_criticality(&self, criticality: TestCriticality) -> Vec<&AssignedTest> {
        self.assigned_tests
            .iter()
            .filter(|t| t.criticality == criticality)
            .collect()
    }

    /// Check if node has a specific capability
    pub fn has_capability(&self, capability: &NodeCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Get monitoring tests (tests with intervals set)
    pub fn get_monitoring_tests(&self) -> Vec<&AssignedTest> {
        self.assigned_tests
            .iter()
            .filter(|t| t.enabled && t.monitor_interval_minutes.is_some())
            .collect()
    }

    /// Get diagnostic-only tests (tests without monitoring intervals)
    pub fn get_diagnostic_tests(&self) -> Vec<&AssignedTest> {
        self.assigned_tests
            .iter()
            .filter(|t| t.enabled && t.monitor_interval_minutes.is_none())
            .collect()
    }
}