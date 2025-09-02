use std::net::{IpAddr, Ipv4Addr};

use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoDiscriminant;
use crate::server::{capabilities::types::{base::{Capability, CapabilityDiscriminants}, configs::NodeConfig}, nodes::types::{criticality::TestCriticality, status::NodeStatus, targets::NodeTarget}, subnets::types::base::{NodeSubnetMembership, Subnet}, tests::types::execution::TestResult};
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
    pub description: Option<String>,
    pub target: NodeTarget,
    pub subnets: Vec<NodeSubnetMembership>,
    
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Node {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub base: NodeBase,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        
        let host_match = match (&self.base.hostname, &other.base.hostname) {
            (Some(host_a), Some(host_b)) => !vec!("localhost".to_string()).contains(&host_a) && host_a == host_b,
            (_, _) => false
        };

        let macs_a: Vec<Option<MacAddress>> = self.base.subnets.iter().map(|s| s.mac_address).collect();
        let macs_b: Vec<Option<MacAddress>> = other.base.subnets.iter().map(|s| s.mac_address).collect();

        let mac_match = macs_a.iter().any(|mac_a| {
            macs_b.iter().any(|mac_b| {
                match (mac_a, mac_b) {
                    (Some(a), Some(b)) => !vec!(
                        MacAddress::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                        MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
                    ).contains(&a) && a == b,
                    (_, _) => false
                }
            })
        });

        return mac_match && host_match
    }
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

    pub fn default_subnet(&self) -> &NodeSubnetMembership {
        &self.base.subnets[0]
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

    pub fn is_gateway_for_subnet(&self, subnet: &mut Subnet) -> bool {
        self.base.subnets.iter().any(|subnet_membership| {
            if subnet_membership.subnet_id == subnet.id {
                 let ip_octets = match subnet_membership.ip_address.to_canonical() {
                    IpAddr::V4(ip) => ip.octets(),
                    IpAddr::V6(ip ) => ip.to_ipv4().unwrap_or(Ipv4Addr::LOCALHOST).octets()
                 };

                 return ip_octets.last() == Some(&u8::from(1)) || ip_octets.last() == Some(&u8::from(254))
            }
            return false
        })
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