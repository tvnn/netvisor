use cidr::{IpCidr};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoDiscriminant;
use DiscoveryPort::*;
use crate::server::{discovery::types::base::DiscoveryPort, nodes::types::{capabilities::{CapabilityConfig, NodeCapability, NodeCapabilityDiscriminants}, criticality::TestCriticality, status::NodeStatus, targets::{IpAddressTargetConfig, NodeTarget}}, tests::types::{base::Test, configs::ConnectivityConfig, execution::TestResult}};
use super::{
    types::{NodeType},
    tests::{AssignedTest},
};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NodeBase {
    pub name: String,
    pub node_type: NodeType,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub target: NodeTarget,
    pub subnets: Vec<IpCidr>,
    
    // Discovery & Capability Data
    pub discovery_status: Option<DiscoveryStatus>,
    pub capabilities: Vec<NodeCapability>,
    
    // Monitoring
    pub status: NodeStatus,
    pub assigned_tests: Vec<AssignedTest>,
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
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            last_seen: None,
            base,
        }
    }

    pub fn default_tests() -> Vec<AssignedTest> {
        vec!(
            AssignedTest {
                test: Test::Connectivity(ConnectivityConfig::default()),
                criticality: TestCriticality::Critical,
            }
        )
    }
    
    // Helper constructor for just a name
    pub fn from_name(name: String) -> Self {
        let base = NodeBase {
            name,
            description: None,
            hostname: None,
            target: NodeTarget::IpAddress(IpAddressTargetConfig::default()),
            node_type: NodeType::UnknownDevice,
            discovery_status: None,
            
            mac_address: None,
            subnets: Vec::new(),
            capabilities: Vec::new(),

            status: NodeStatus::Unknown,
            assigned_tests: Node::default_tests(),
            monitoring_interval: 5,
            node_groups: Vec::new(),
        };
        Self::new(base)
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

    pub fn has_capability(&self, capability_discriminant: NodeCapabilityDiscriminants) -> bool{
        self.base.capabilities.iter().any(|c| c.discriminant() == capability_discriminant)
    }

    pub fn get_capability(&self, capability_discriminant: NodeCapabilityDiscriminants) -> Option<&NodeCapability>{
        self.base.capabilities.iter().find(|c| c.discriminant() == capability_discriminant)
    }

    pub fn add_capability_from_port(&mut self, port: u16) {
        let capability = match DiscoveryPort::try_from(port).ok() {
            Some(Ssh) => NodeCapability::SshAccess {
                config: CapabilityConfig::from_port(port),
            },
            Some(Dns) => NodeCapability::DnsService {
                config: CapabilityConfig::from_port(port),
            },
            Some(Http | HttpAlt) => NodeCapability::HttpService {
                config: CapabilityConfig::from_port(port),
            },
            Some(Https | HttpsAlt) => NodeCapability::HttpsService {
                config: CapabilityConfig::from_port(port),
            },
            Some(WireGuard) => NodeCapability::WireGuardService {
                config: CapabilityConfig::from_port(port),
            },
            Some(OpenVpn | Pptp) => NodeCapability::OpenVpnService {
                config: CapabilityConfig::from_port(port),
            },
            Some(IpsecIke | IpsecNat) => NodeCapability::IpsecService {
                config: CapabilityConfig::from_port(port),
            },
            Some(Snmp | SnmpTrap) => NodeCapability::SnmpService {
                config: CapabilityConfig::from_port(port),
            },
            Some(Rdp) => NodeCapability::RdpService {
                config: CapabilityConfig::from_port(port),
            },
            Some(Telnet) => NodeCapability::TelnetService {
                config: CapabilityConfig::from_port(port),
            },
            Some(Dhcp) => NodeCapability::DhcpService {
                config: CapabilityConfig::from_port(port),
            },
            None => return,
        };
        
        // Check if we already have this capability type and update it, or add new
        if let Some(existing) = self.base.capabilities.iter_mut().find(|cap| {
            std::mem::discriminant(*cap) == std::mem::discriminant(&capability)
        }) {
            existing.config_mut().set_port(port);
        } else {
            self.base.capabilities.push(capability);
        }
    }

    // pub fn add_capability_from_process(&mut self, process_name: &str) {
    //     let service_capabilities: Vec<NodeCapability> = match process_name.to_lowercase().as_str() {
    //         name if name.contains("wg") || name.contains("wireguard") => 
    //             vec![NodeCapability::WireGuardService {
    //                 config: CapabilityConfig::from_process(process_name.to_string()),
    //             }],
    //         name if name.contains("openvpn") => 
    //             vec![NodeCapability::OpenVpnService {
    //                 config: CapabilityConfig::from_process(process_name.to_string()),
    //             }],
    //         name if name.contains("nginx") || name.contains("apache") || name.contains("httpd") => 
    //             vec![
    //                 NodeCapability::HttpService {
    //                     config: CapabilityConfig::from_process(process_name.to_string()),
    //                 },
    //                 NodeCapability::HttpsService {
    //                     config: CapabilityConfig::from_process(process_name.to_string()),
    //                 }
    //             ],
    //         name if name.contains("sshd") => 
    //             vec![NodeCapability::SshAccess {
    //                 config: CapabilityConfig::from_process(process_name.to_string()),
    //             }],
    //         _ => vec![],
    //     };
        
    //     for capability in service_capabilities {
    //         // Check if we already have this capability type and update it, or add new
    //         if let Some(existing) = self.base.capabilities.iter_mut().find(|cap| {
    //             std::mem::discriminant(*cap) == std::mem::discriminant(&capability)
    //         }) {
    //             existing.config_mut().set_process(process_name.to_string());
    //         } else {
    //             self.base.capabilities.push(capability);
    //         }
    //     }
    // }
}