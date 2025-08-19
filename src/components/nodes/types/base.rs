use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;
use std::mem::discriminant;
use crate::components::{nodes::types::{targets::NodeTarget, tests::TestTypeCompatibilityInfo}, tests::types::{Test, TestResult}};
use crate::shared::types::ApplicationProtocol;
use super::{
    types_capabilities::{NodeType, NodeCapability},
    tests::{AssignedTest, NodeStatus, TestCriticality},
    topology::{GraphPosition}
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NodeBase {
    pub name: String,
    pub node_type: NodeType,
    pub description: Option<String>,
    pub target: NodeTarget,
    
    // Discovery & Capability Data
    pub open_ports: Vec<u16>,
    pub detected_services: Vec<DetectedService>,
    pub mac_address: Option<String>,
    pub capabilities: Vec<NodeCapability>,
    
    // Monitoring
    pub assigned_tests: Vec<AssignedTest>,
    pub monitoring_interval: u16,
    pub node_groups: Vec<String>,
    
    // Topology visualization
    pub position: Option<GraphPosition>,
    pub current_status: NodeStatus,
    pub subnet_membership: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
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
            description: None,
            target: NodeTarget::ip_template(),
            node_type: NodeType::UnknownDevice,
            
            open_ports: Vec::new(),
            detected_services: Vec::new(),
            mac_address: None,
            capabilities: Vec::new(),

            assigned_tests: Vec::new(),
            monitoring_interval: 5,
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
                match result.criticality {
                    Some(TestCriticality::Critical) => has_critical_failure = true,
                    Some(TestCriticality::Important) => has_important_failure = true,
                    Some(TestCriticality::Informational) => {}, // Doesn't affect status
                    None => {}
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

    /// Get compatible test types for a node
    pub fn get_compatible_test_types(&self) -> (Vec<TestTypeCompatibilityInfo>, Vec<TestTypeCompatibilityInfo>) {

        let mut recommended_tests = Vec::new();
        let mut warned_tests = Vec::new();

        for test_default in Test::iter() {
            let is_assigned = self.base.assigned_tests.iter().any(|a| discriminant(&a.test) == discriminant(&test_default));
            let warning = test_default.check_node_compatibility(&self);

            let test_info = TestTypeCompatibilityInfo {
                test_type: test_default.variant_name(),
                display_name: test_default.display_name().to_string(),
                description: test_default.description().to_string(),
                contextual_description: test_default.generate_context_description(&self).to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct DetectedService {
    pub port: u16,
    pub protocol: ApplicationProtocol,
    pub service_name: Option<String>, // "nginx", "OpenSSH", "MySQL"
    pub version: Option<String>, // "1.20.1", "8.9p1", "8.0.32"
    pub banner: Option<String>,  // Raw service banner
}
