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
pub struct TestRecommendation {
    pub ideal_node_types: Vec<NodeType>,
    pub helpful_capabilities: Vec<NodeCapability>,
    pub warning_message: Option<String>,
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

    /// Get recommendations for this test type (replaces hard restrictions)
    pub fn get_recommendations(&self) -> TestRecommendation {
        match self {
            TestType::VpnConnectivity => TestRecommendation {
                ideal_node_types: vec![NodeType::VpnServer],
                helpful_capabilities: vec![],
                warning_message: Some("VPN connectivity tests work best with VPN servers, but can test any network endpoint".to_string()),
            },
            TestType::VpnTunnel => TestRecommendation {
                ideal_node_types: vec![NodeType::VpnServer],
                helpful_capabilities: vec![],
                warning_message: Some("VPN tunnel tests are designed for VPN servers and may not work as expected on other device types".to_string()),
            },
            TestType::ServiceHealth => TestRecommendation {
                ideal_node_types: vec![NodeType::WebServer],
                helpful_capabilities: vec![NodeCapability::HttpService, NodeCapability::HttpsService],
                warning_message: Some("Service health tests work best with web servers, but can test any HTTP/HTTPS endpoint".to_string()),
            },
            TestType::DnsResolution => TestRecommendation {
                ideal_node_types: vec![NodeType::DnsServer],
                helpful_capabilities: vec![NodeCapability::DnsService],
                warning_message: None, // DNS resolution can be tested from any device
            },
            TestType::DnsOverHttps => TestRecommendation {
                ideal_node_types: vec![],
                helpful_capabilities: vec![],
                warning_message: None, // DoH can be tested from any device
            },
            TestType::DaemonCommand => TestRecommendation {
                ideal_node_types: vec![
                    NodeType::Router, NodeType::WebServer, NodeType::DatabaseServer, 
                    NodeType::MediaServer, NodeType::DnsServer, NodeType::VpnServer, 
                    NodeType::NasDevice, NodeType::Workstation
                ],
                helpful_capabilities: vec![],
                warning_message: Some("Daemon commands require NetFrog daemon installation and may not work on IoT devices, printers, or cameras".to_string()),
            },
            TestType::SshScript => TestRecommendation {
                ideal_node_types: vec![
                    NodeType::Router, NodeType::WebServer, NodeType::DatabaseServer, 
                    NodeType::MediaServer, NodeType::DnsServer, NodeType::VpnServer, 
                    NodeType::NasDevice, NodeType::Workstation
                ],
                helpful_capabilities: vec![NodeCapability::SshAccess],
                warning_message: Some("SSH tests require SSH access and may not work on IoT devices, printers, or cameras".to_string()),
            },
            // Basic connectivity tests work with any device
            TestType::Connectivity | TestType::DirectIp | TestType::Ping | TestType::WellknownIp => {
                TestRecommendation {
                    ideal_node_types: vec![],
                    helpful_capabilities: vec![],
                    warning_message: None,
                }
            },
        }
    }

    /// Check if this test is compatible (now always returns true, but provides warning info)
    pub fn is_compatible_with_node(&self, _node: &Node) -> bool {
        // Always allow assignment - we'll show warnings in the UI instead
        true
    }

    /// Get warning message for assigning this test to a specific node
    pub fn get_assignment_warning(&self, node: &Node) -> Option<String> {
        let recommendations = self.get_recommendations();
        
        // Check if this is an ideal assignment
        if let Some(node_type) = &node.base.node_type {
            if recommendations.ideal_node_types.contains(node_type) {
                return None; // Perfect match, no warning
            }
        }
        
        // Check if node has helpful capabilities
        let has_helpful_caps = recommendations.helpful_capabilities.is_empty() || 
            recommendations.helpful_capabilities.iter().any(|cap| node.base.capabilities.contains(cap));
        
        // Generate warning based on context
        match self {
            TestType::VpnConnectivity | TestType::VpnTunnel => {
                if !matches!(node.base.node_type, Some(NodeType::VpnServer)) {
                    Some(format!("⚠️ {} tests are typically used with VPN servers. This will test basic connectivity but may not provide VPN-specific insights.", self.display_name()))
                } else {
                    None
                }
            },
            TestType::ServiceHealth => {
                if !has_helpful_caps && !matches!(node.base.node_type, Some(NodeType::WebServer)) {
                    Some("⚠️ Service health tests work best with web services. Ensure the target has an HTTP/HTTPS endpoint.".to_string())
                } else {
                    None
                }
            },
            TestType::DaemonCommand => {
                if matches!(node.base.node_type, Some(NodeType::Printer | NodeType::Camera | NodeType::IotDevice)) {
                    Some("⚠️ Daemon commands may not work on this device type. Ensure NetFrog daemon can be installed.".to_string())
                } else {
                    None
                }
            },
            TestType::SshScript => {
                if !node.base.capabilities.contains(&NodeCapability::SshAccess) {
                    Some("⚠️ SSH tests require SSH access. Ensure this device accepts SSH connections.".to_string())
                } else {
                    None
                }
            },
            _ => None, // No warnings for basic connectivity tests
        }
    }

    /// Generate contextual description for this test on a specific node
    pub fn generate_context_description(&self, node: &Node) -> String {
        let node_type_str = node.base.node_type
            .as_ref()
            .map(|t| t.display_name())
            .unwrap_or("device");

        match self {
            TestType::Connectivity => format!("Can we connect to your {}?", node_type_str),
            TestType::DirectIp => format!("Can we reach your {} directly by IP?", node_type_str),
            TestType::Ping => format!("Can we ping your {}?", node_type_str),
            TestType::WellknownIp => "Can we reach well-known internet services?".to_string(),
            TestType::DnsResolution => {
                if matches!(node.base.node_type, Some(NodeType::DnsServer)) {
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