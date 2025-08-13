use serde::{Deserialize, Serialize};
use crate::core::node_types::{NodeType, NodeCapability};
use crate::core::types::Node;
use crate::components::tests::configs::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TestType {
    // Basic Connectivity (VPN-focused subset)
    Connectivity,
    DirectIp,
    Ping,
    WellknownIp,
    
    // DNS Resolution  
    DnsResolution,
    DnsOverHttps,
    
    // VPN Connectivity
    VpnConnectivity,
    VpnTunnel,
    
    // Service Health
    ServiceHealth,
    
    // Future daemon-based tests (Phase 5)
    DaemonCommand,
    SshScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCapabilityRequirements {
    pub required_capabilities: Vec<NodeCapability>,
    pub required_node_types: Vec<NodeType>,
    pub forbidden_node_types: Vec<NodeType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TestCriticality {
    Critical,        // Failure = NodeStatus::Failed
    Important,       // Failure = NodeStatus::Degraded  
    Informational,   // Failure = NodeStatus::Healthy (just logged)
}

// Base configuration shared by all test types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTestConfig {
    pub timeout: Option<u64>,
    pub expected_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestConfiguration {
    Connectivity(ConnectivityConfig),
    DirectIp(DirectIpConfig),
    Ping(PingConfig),
    WellknownIp(WellknownIpConfig),
    DnsResolution(DnsResolutionConfig),
    DnsOverHttps(DnsOverHttpsConfig),
    VpnConnectivity(VpnConnectivityConfig),
    VpnTunnel(VpnTunnelConfig),
    ServiceHealth(ServiceHealthConfig),
    DaemonCommand(DaemonCommandConfig),
    SshScript(SshScriptConfig),
}

impl TestType {
    /// Get display name for this test type
    pub fn display_name(&self) -> &'static str {
        match self {
            TestType::Connectivity => "Connectivity",
            TestType::DirectIp => "Direct IP",
            TestType::Ping => "Ping",
            TestType::WellknownIp => "Well-known IP",
            TestType::DnsResolution => "DNS Resolution",
            TestType::DnsOverHttps => "DNS over HTTPS",
            TestType::VpnConnectivity => "VPN Connectivity",
            TestType::VpnTunnel => "VPN Tunnel",
            TestType::ServiceHealth => "Service Health",
            TestType::DaemonCommand => "Daemon Command",
            TestType::SshScript => "SSH Script",
        }
    }

    /// Get capability requirements for this test type
    pub fn capability_requirements(&self) -> Option<TestCapabilityRequirements> {
        match self {
            // VPN tests require VPN-capable nodes
            TestType::VpnConnectivity | TestType::VpnTunnel => Some(TestCapabilityRequirements {
                required_capabilities: vec![],
                required_node_types: vec![NodeType::VpnServer],
                forbidden_node_types: vec![NodeType::Printer, NodeType::Camera, NodeType::IotDevice],
            }),
            
            // DNS resolution tests work best with DNS servers but can run on any network-capable device
            TestType::DnsResolution | TestType::DnsOverHttps => Some(TestCapabilityRequirements {
                required_capabilities: vec![],
                required_node_types: vec![], // Allow on any node type
                forbidden_node_types: vec![NodeType::Printer], // Printers typically can't do DNS queries
            }),
            
            // Service health tests require HTTP/HTTPS capabilities
            TestType::ServiceHealth => Some(TestCapabilityRequirements {
                required_capabilities: vec![NodeCapability::HttpService], // OR NodeCapability::HttpsService
                required_node_types: vec![],
                forbidden_node_types: vec![NodeType::Printer, NodeType::Camera],
            }),
            
            // Daemon/SSH tests require remote access capabilities
            TestType::DaemonCommand => Some(TestCapabilityRequirements {
                required_capabilities: vec![], // Daemon will be installed separately
                required_node_types: vec![NodeType::Router, NodeType::WebServer, NodeType::DatabaseServer, 
                                         NodeType::MediaServer, NodeType::DnsServer, NodeType::VpnServer, 
                                         NodeType::NasDevice, NodeType::Workstation],
                forbidden_node_types: vec![NodeType::Printer, NodeType::Camera, NodeType::IotDevice],
            }),
            
            TestType::SshScript => Some(TestCapabilityRequirements {
                required_capabilities: vec![NodeCapability::SshAccess],
                required_node_types: vec![],
                forbidden_node_types: vec![NodeType::Printer, NodeType::Camera, NodeType::IotDevice],
            }),
            
            // Basic connectivity tests can run against any network-accessible device
            TestType::Connectivity | TestType::DirectIp | TestType::Ping | TestType::WellknownIp => None,
        }
    }
    
    /// Check if this test type is compatible with a node
    pub fn is_compatible_with_node(&self, node: &Node) -> bool {
        if let Some(requirements) = self.capability_requirements() {
            // Check forbidden node types
            if let Some(node_type) = &node.node_type {
                if requirements.forbidden_node_types.contains(node_type) {
                    return false;
                }
            }
            
            // Check required node types (if specified)
            if !requirements.required_node_types.is_empty() {
                if let Some(node_type) = &node.node_type {
                    if !requirements.required_node_types.contains(node_type) {
                        return false;
                    }
                } else {
                    return false; // Required node type but node has no type
                }
            }
            
            // Check required capabilities (if specified)
            if !requirements.required_capabilities.is_empty() {
                for required_cap in &requirements.required_capabilities {
                    if !node.capabilities.contains(required_cap) {
                        return false;
                    }
                }
            }
        }
        
        true
    }

    /// Generate contextual test description
    pub fn generate_description(&self, node: &Node) -> String {
        let node_type_str = node.node_type.as_ref()
            .map(|t| t.display_name().to_lowercase())
            .unwrap_or("device".to_string());
            
        match self {
            TestType::Connectivity => format!("Can we reach your {}?", node_type_str),
            TestType::DirectIp => format!("Can we connect directly to your {}?", node_type_str),
            TestType::Ping => format!("Can we ping your {}?", node_type_str),
            TestType::WellknownIp => "Can we reach well-known internet services?".to_string(),
            TestType::DnsResolution => {
                if matches!(node.node_type, Some(NodeType::DnsServer)) {
                    format!("Can we resolve names using your {}?", node_type_str)
                } else {
                    format!("Can your {} resolve DNS names?", node_type_str)
                }
            },
            TestType::DnsOverHttps => format!("Can your {} use DNS over HTTPS?", node_type_str),
            TestType::VpnConnectivity => format!("Can we reach your {}?", node_type_str),
            TestType::VpnTunnel => format!("Is your {} tunnel working?", node_type_str),
            TestType::ServiceHealth => format!("Is your {} responding properly?", node_type_str),
            TestType::DaemonCommand => format!("Execute command on {}", node_type_str),
            TestType::SshScript => format!("Run script via SSH on {}", node_type_str),
        }
    }
}

impl TestCriticality {
    pub fn display_name(&self) -> &'static str {
        match self {
            TestCriticality::Critical => "Critical",
            TestCriticality::Important => "Important",
            TestCriticality::Informational => "Informational",
        }
    }
}

impl Default for BaseTestConfig {
    fn default() -> Self {
        Self {
            timeout: Some(30000), // 30 seconds default
            expected_result: "Success".to_string(),
        }
    }
}