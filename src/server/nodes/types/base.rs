use cidr::{IpCidr};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoDiscriminant;
use crate::server::{capabilities::types::{base::{Capability, CapabilityDiscriminants}, configs::{NodeConfig}}, nodes::types::{criticality::TestCriticality, status::NodeStatus, targets::{NodeTarget}}, tests::types::execution::TestResult};
use super::{
    types::{NodeType},
};
use uuid::{Uuid};
use crate::server::shared::types::api::deserialize_empty_string_as_none;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NodeBase {
    pub name: String,
    pub node_type: NodeType,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub hostname: Option<String>,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub mac_address: Option<String>,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub description: Option<String>,
    pub target: NodeTarget,
    pub subnets: Vec<IpCidr>,
    
    // Discovery & Capability Data
    pub discovery_status: Option<DiscoveryStatus>,
    pub capabilities: Vec<Capability>,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub dns_resolver_node_id: Option<String>,
    
    // Monitoring
    pub status: NodeStatus,
    pub monitoring_interval: u16,
    pub node_groups: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum DiscoveryStatus {
    Discovered { session_id: Uuid, discovered_at: DateTime<Utc> },
    Reviewed,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Node {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub base: NodeBase,
}

impl Node {
    pub fn new(base: NodeBase) -> Self {
        let now = chrono::Utc::now();
        let mut node = Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            last_seen: None,
            base,
        };
        if !node.has_capability(CapabilityDiscriminants::Node){
            node.add_capability(
                Capability::Node(NodeConfig::new(&node))
            );
        }

        node
    }

    pub fn as_context(&self) -> NodeContext {
        NodeContext { 
            node_id: Some(self.id), 
            node_type: self.base.node_type.clone(), 
            capabilities: self.base.capabilities.clone(), 
            target: self.base.target.clone()
        }
    }
    
    // Node group management
    pub fn add_to_group(&mut self, group_id: Uuid) {
        if !self.base.node_groups.contains(&group_id) {
            self.base.node_groups.push(group_id);
            self.updated_at = chrono::Utc::now();
        }
    }
    
    pub fn remove_from_group(&mut self, group_id: &Uuid) {
        self.base.node_groups.retain(|id| id != group_id);
        self.updated_at = chrono::Utc::now();
    }

    /// Compute and update node status based on test results
    pub fn update_status_from_tests(&mut self, test_results: &[TestResult]) {        
        
        if test_results.is_empty() {
            self.base.status = NodeStatus::Unknown;
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
        
        self.base.status = new_status;
    }

    pub fn has_capability(&self, capability_discriminant: CapabilityDiscriminants) -> bool{
        self.base.capabilities.iter().any(|c| c.discriminant() == capability_discriminant)
    }

    pub fn get_capability(&self, capability_discriminant: CapabilityDiscriminants) -> Option<&Capability>{
        self.base.capabilities.iter().find(|c| c.discriminant() == capability_discriminant)
    }

    pub fn add_capability(&mut self, capability: Capability) {        
        self.base.capabilities.push(capability);
    }
}

// Used during node form to generate capability form based on provided but unsaved information
#[derive(Debug, Clone, Deserialize)]
pub struct NodeContext {
    pub node_id: Option<Uuid>,
    pub node_type: NodeType,
    pub capabilities: Vec<Capability>,
    pub target: NodeTarget,
}

impl NodeContext {
    pub fn has_capability(&self, capability_discriminant: CapabilityDiscriminants) -> bool{
        self.capabilities.iter().any(|c| c.discriminant() == capability_discriminant)
    }
}