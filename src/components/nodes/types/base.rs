use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;
use crate::components::{nodes::types::tests::{TestTypeCompatibilityInfo}, tests::types::{TestResult, TestType}};
use super::{
    types_capabilities::{NodeType, NodeCapability},
    tests::{AssignedTest, NodeStatus, TestCriticality},
    topology::{GraphPosition}
};

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

    /// Compute and update node status based on test results
    pub fn update_status_from_tests(&mut self, test_results: &[TestResult]) {        
        
        if test_results.is_empty() {
            self.base.current_status = NodeStatus::Unknown;
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
        
        self.base.current_status = new_status;
    }

    /// Get the target for tests (IP, domain, or name in preference order)
    pub fn get_target(&self) -> String {
        self.base.ip.clone()
            .or_else(|| self.base.domain.clone())
            .unwrap_or_else(|| self.base.name.to_string())
    }

    /// Get compatible test types for a node
    pub fn get_compatible_test_types(&self) -> (Vec<TestTypeCompatibilityInfo>, Vec<TestTypeCompatibilityInfo>) {

        let mut recommended_tests = Vec::new();
        let mut warned_tests = Vec::new();

        for test_type in TestType::iter() {
            let is_assigned = self.base.assigned_tests.iter().any(|t| t.test_type == test_type);
            let warning = test_type.get_assignment_warning(&self);

            let test_info = TestTypeCompatibilityInfo {
                test_type: test_type.clone(),
                display_name: test_type.display_name().to_string(),
                description: test_type.description().to_string(),
                contextual_description: test_type.generate_context_description(&self).to_string(),
                is_assigned,
                warning: warning.clone(),
                is_recommended: warning.is_none(),
            };
            
            if warning.is_none() {
                recommended_tests.push(test_info);
            } else {
                warned_tests.push(test_info);
            }
        }

        (recommended_tests, warned_tests)

    }
}

