use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::node_types::{NodeType, NodeCapability};
use crate::core::test_types::{TestType, TestConfiguration, TestCriticality};

// Node Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeBase {
    pub name: String,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub description: Option<String>,
    
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub base: NodeBase,
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
pub struct NodeGroupBase {
    pub name: String,
    pub description: Option<String>,
    pub node_sequence: Vec<String>,  // Ordered diagnostic sequence
    pub auto_diagnostic_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGroup {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: NodeGroupBase,
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
    pub fn new(base: NodeBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            last_seen: None,
            base,
        }
    }
    
    // Helper constructor for just a name
    pub fn from_name(name: String) -> Self {
        let base = NodeBase {
            name,
            domain: None,
            ip: None,
            port: None,
            path: None,
            description: None,
            node_type: None,
            capabilities: Vec::new(),
            assigned_tests: Vec::new(),
            monitoring_enabled: false,
            node_groups: Vec::new(),
            position: None,
            current_status: NodeStatus::Unknown,
            subnet_membership: Vec::new(),
        };
        Self::new(base)
    }

    // Setters that handle side effects (timestamps)
    pub fn set_node_type(&mut self, node_type: Option<NodeType>) {
        self.base.node_type = node_type;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn set_monitoring_enabled(&mut self, enabled: bool) {
        self.base.monitoring_enabled = enabled;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn set_current_status(&mut self, status: NodeStatus) {
        self.base.current_status = status;
        self.updated_at = chrono::Utc::now();
    }

    pub fn set_last_seen(&mut self, last_seen: DateTime<Utc>) {
        self.last_seen = Some(last_seen);
        self.updated_at = chrono::Utc::now();
    }

    // Test management methods
    pub fn assign_test(&mut self, assigned_test: AssignedTest) {
        // Remove existing test of the same type if it exists
        self.base.assigned_tests.retain(|t| t.test_type != assigned_test.test_type);
        self.base.assigned_tests.push(assigned_test);
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn remove_test(&mut self, test_type: &TestType) -> bool {
        let initial_len = self.base.assigned_tests.len();
        self.base.assigned_tests.retain(|t| &t.test_type != test_type);
        if self.base.assigned_tests.len() != initial_len {
            self.updated_at = chrono::Utc::now();
            true
        } else {
            false
        }
    }

    /// Update specific fields of an assigned test
    pub fn update_test_fields(
        &mut self, 
        test_type: &TestType,
        test_config: Option<TestConfiguration>,
        criticality: Option<TestCriticality>,
        monitor_interval_minutes: Option<Option<u32>>,
        enabled: Option<bool>,
    ) -> Result<(), String> {
        if let Some(test) = self.base.assigned_tests.iter_mut().find(|t| &t.test_type == test_type) {
            if let Some(config) = test_config {
                test.test_config = config;
            }
            if let Some(crit) = criticality {
                test.criticality = crit;
            }
            if let Some(interval) = monitor_interval_minutes {
                test.monitor_interval_minutes = interval;
            }
            if let Some(en) = enabled {
                test.enabled = en;
            }
            
            self.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(format!("Test {} not found", test_type.display_name()))
        }
    }
    
    // Node group management
    pub fn add_to_group(&mut self, group_id: String) {
        if !self.base.node_groups.contains(&group_id) {
            self.base.node_groups.push(group_id);
            self.updated_at = chrono::Utc::now();
        }
    }
    
    pub fn remove_from_group(&mut self, group_id: &str) {
        self.base.node_groups.retain(|id| id != group_id);
        self.updated_at = chrono::Utc::now();
    }

    // Combined operations
    pub fn update_status_and_last_seen(&mut self, status: &NodeStatus) {
        self.base.current_status = status.clone();
        self.last_seen = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }

    /// Compute and update node status based on test results
    pub fn compute_status_from_tests(&mut self, test_results: &[TestResult]) {
        use crate::core::test_types::TestCriticality;
        
        if test_results.is_empty() {
            self.set_current_status(NodeStatus::Unknown);
            return;
        }
        
        let mut has_critical_failure = false;
        let mut has_important_failure = false;
        
        for result in test_results {
            if !result.success {
                if let Some(assigned_test) = self.base.assigned_tests.iter().find(|t| t.test_type == result.test_type) {
                    match assigned_test.criticality {
                        TestCriticality::Critical => has_critical_failure = true,
                        TestCriticality::Important => has_important_failure = true,
                        TestCriticality::Informational => {}, // Doesn't affect status
                    }
                }
            }
        }
        
        let new_status = if has_critical_failure {
            NodeStatus::Failed
        } else if has_important_failure {
            NodeStatus::Degraded
        } else {
            NodeStatus::Healthy
        };
        
        self.set_current_status(new_status);
    }

    /// Get the target for tests (IP, domain, or name in preference order)
    pub fn get_target(&self) -> String {
        self.base.ip.clone()
            .or_else(|| self.base.domain.clone())
            .unwrap_or_else(|| self.base.name.to_string())
    }
}

impl NodeGroup {
    pub fn new(base: NodeGroupBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            base,
        }
    }

    pub fn from_name(name: String) -> Self {
        let base = NodeGroupBase {
            name,
            description: None,
            node_sequence: Vec::new(),
            auto_diagnostic_enabled: true,
        };

        Self::new(base)
    }

    // Setters with timestamp updates
    pub fn set_auto_diagnostic_enabled(&mut self, enabled: bool) {
        self.base.auto_diagnostic_enabled = enabled;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn set_description(&mut self, description: String) {
        self.base.description = Some(description);
        self.updated_at = chrono::Utc::now();
    }

    // Node sequence management
    pub fn add_node(&mut self, node_id: String) {
        if !self.base.node_sequence.contains(&node_id) {
            self.base.node_sequence.push(node_id);
            self.updated_at = chrono::Utc::now();
        }
    }
    
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        let initial_len = self.base.node_sequence.len();
        self.base.node_sequence.retain(|id| id != node_id);
        if self.base.node_sequence.len() != initial_len {
            self.updated_at = chrono::Utc::now();
            true
        } else {
            false
        }
    }
    
    pub fn reorder_nodes(&mut self, new_sequence: Vec<String>) {
        self.base.node_sequence = new_sequence;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn move_node_up(&mut self, node_id: &str) -> bool {
        if let Some(index) = self.base.node_sequence.iter().position(|id| id == node_id) {
            if index > 0 {
                self.base.node_sequence.swap(index - 1, index);
                self.updated_at = chrono::Utc::now();
                return true;
            }
        }
        false
    }
    
    pub fn move_node_down(&mut self, node_id: &str) -> bool {
        if let Some(index) = self.base.node_sequence.iter().position(|id| id == node_id) {
            if index < self.base.node_sequence.len() - 1 {
                self.base.node_sequence.swap(index, index + 1);
                self.updated_at = chrono::Utc::now();
                return true;
            }
        }
        false
    }

    // Read-only methods (no setters needed)
    pub fn contains_node(&self, node_id: &str) -> bool {
        self.base.node_sequence.contains(&node_id.to_string())
    }

    pub fn node_count(&self) -> usize {
        self.base.node_sequence.len()
    }
}