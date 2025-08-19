use std::{net::{IpAddr, Ipv4Addr}, time::Instant};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use crate::{components::{nodes::types::{
    base::{Node, NodeTarget}, tests::TestCriticality, types_capabilities::{NodeCapability, NodeType}
}, tests::implementations::*}, shared::types::TransportProtocol};
use chrono::{DateTime, Utc};
use cidr::{IpCidr, Ipv4Cidr};
use strum_macros::{EnumIter, EnumDiscriminants, Display};
use std::mem::discriminant;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum Test {
    // Basic Connectivity Tests
    Connectivity(ConnectivityConfig),
    DirectIp(DirectIpConfig),
    Ping(PingConfig),
    
    // Service-Specific Tests
    ServiceHealth(ServiceHealthConfig),
    DnsResolution(DnsResolutionConfig),
    DnsLookup(DnsLookupConfig),
    DnsOverHttps(DnsOverHttpsConfig),
    ReverseDns(ReverseDnsConfig),
    
    // VPN-Specific Tests
    VpnConnectivity(VpnConnectivityConfig),
    VpnTunnel(VpnTunnelConfig),
        
    // Remote tests
    // DaemonCommand(DaemonCommandConfig),
    // SshScript(SshScriptConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct TestRequirements {
    pub node_types: Option<Vec<NodeType>>,
    pub node_capabilities: Option<Vec<NodeCapability>>,
    pub node_target_types: Option<Vec<NodeTarget>>,
    pub warning_message: Option<String>,
}

impl Test {
    pub fn variant_name(&self) -> String {
        TestDiscriminants::from(self).to_string()
    }
    
    /// Get display name for this test type
    pub fn display_name(&self) -> &str {
        match self {
            Test::Connectivity(_) => "Connectivity",
            Test::DirectIp(_) => "Direct IP",
            Test::Ping(_) => "Ping",
            Test::DnsResolution(_) => "DNS Resolution",
            Test::DnsOverHttps(_) => "DNS over HTTPS",
            Test::DnsLookup(_) => "DNS Lookup",
            Test::ReverseDns(_) => "Reverse DNS Lookup",
            Test::VpnConnectivity(_) => "VPN Connectivity",
            Test::VpnTunnel(_) => "VPN Tunnel",
            Test::ServiceHealth(_) => "Service Health",
            // Test::DaemonCommand(_) => "Daemon Command",
            // Test::SshScript(_) => "SSH Script",
        }
    }

    pub fn description(&self) -> &str {
        match &self {
            Test::Connectivity(_) => "Test TCP connectivity to a target host and port",
            Test::DirectIp(_) => "Test direct IP connectivity bypassing DNS resolution",
            Test::Ping(_) => "Test network reachability using ICMP ping",
            Test::DnsResolution(_) => "Test DNS name resolution capabilities",
            Test::DnsOverHttps(_) => "Test DNS resolution using DNS over HTTPS",
            Test::DnsLookup(_) => "Test whether domain can be resolved to IP via DNS",
            Test::ReverseDns(_) => "Test whether IP can be resolved to domain via DNS",
            Test::VpnConnectivity(_) => "Test VPN server reachability and connection",
            Test::VpnTunnel(_) => "Test VPN tunnel functionality and subnet access",
            Test::ServiceHealth(_) => "Test HTTP/HTTPS service health and response",
            // Test::DaemonCommand(_) => "Execute system commands via daemon",
            // Test::SshScript(_) => "Execute commands via SSH connection",
        }
    }

    /// Generate contextual description for this test on a specific node
    pub fn generate_context_description(&self, node: &Node) -> String {
        let node_type_display_name = node.base.node_type.display_name();

        match self {
            Test::Connectivity(_) => format!("Can we connect to your {}?", node_type_display_name),
            Test::DirectIp(_) => format!("Can we reach your {} directly by IP?", node_type_display_name),
            Test::Ping(_) => format!("Can we ping your {}?", node_type_display_name),
            Test::DnsResolution(_) => format!("Can your {} resolve DNS?", node_type_display_name),
            Test::DnsOverHttps(_) => format!("Can your {} resolve DNS over HTTPS?", node_type_display_name),
            Test::DnsLookup(_) => format!("Can your {} domain be resolved to a target IP?", node_type_display_name),
            Test::ReverseDns(_) => format!("Can your {} IP be resolved to a target domain?", node_type_display_name),
            Test::VpnConnectivity(_) => format!("Can we reach your {}?", node_type_display_name),
            Test::VpnTunnel(_) => format!("Is your {} tunnel working?", node_type_display_name),
            Test::ServiceHealth(_) => format!("Is your {} responding properly?", node_type_display_name),
            // Test::DaemonCommand(_) => format!("Execute command on {}", node_type_display_name),
            // Test::SshScript(_) => format!("Run script via SSH on {}", node_type_display_name),
        }
    }

    pub async fn execute(&self, timer: &Timer, node: &Node) -> Result<TestResult, Error> {
        match self {
            Test::Connectivity(config) => connectivity::execute_connectivity_test(config, &timer, &node).await,
            Test::DirectIp(config) => connectivity::execute_direct_ip_test(config, &timer, &node).await,
            Test::Ping(config) => connectivity::execute_ping_test(config, &timer, &node).await,
            Test::DnsResolution(config) => dns::execute_dns_resolution_test(config, &timer, &node).await,
            Test::DnsOverHttps(config) => dns::execute_dns_over_https_test(config, &timer, &node).await,
            Test::DnsLookup(config) => dns::execute_dns_lookup_test(config, &timer, &node).await,
            Test::ReverseDns(config) => dns::execute_reverse_dns_lookup_test(config, &timer, &node).await,
            Test::VpnConnectivity(config) => vpn::execute_vpn_connectivity_test(config, &timer, &node).await,
            Test::VpnTunnel(config) => vpn::execute_vpn_tunnel_test(config, &timer, &node).await,
            Test::ServiceHealth(config) => service::execute_service_health_test(config, &timer, &node).await
        }
    }

    /// Get requirements for this test type
    pub fn get_requirements(&self) -> TestRequirements {
        match self {
            Test::Connectivity(_) => TestRequirements {
                node_types: None,
                node_capabilities: None,
                node_target_types: None,
                warning_message: None,
            },
            
            Test::DirectIp(_) => TestRequirements {
                node_types: None,
                node_capabilities: None,
                node_target_types: Some(vec![
                    NodeTarget::ipv4_template(),
                    NodeTarget::ipv6_template()
                ]),
                warning_message: Some("DirectIp test requires a node with an IP address target".to_string()),
            },
            
            Test::Ping(_) => TestRequirements {
                node_types: None,
                node_capabilities: None,
                node_target_types: Some(vec![
                    NodeTarget::ipv4_template(),
                    NodeTarget::ipv6_template(),
                    NodeTarget::hostname_template()
                ]),
                warning_message: Some("Ping test requires a node with IP address or hostname target".to_string()),
            },
            
            Test::ServiceHealth(_) => TestRequirements {
                node_types: None,
                node_capabilities: Some(vec![
                    NodeCapability::HttpService,
                    NodeCapability::HttpsService
                ]),
                node_target_types: Some(vec![NodeTarget::service_template()]),
                warning_message: Some("ServiceHealth test requires a node with HTTP/HTTPS capability and service target configuration".to_string()),
            },
            
            Test::DnsResolution(_) => TestRequirements {
                node_types: Some(vec![NodeType::DnsServer]),
                node_capabilities: Some(vec![
                    NodeCapability::DnsService,
                ]),
                node_target_types: None,
                warning_message: Some("DnsResolution test should be assigned to DNS servers with DNS capability".to_string()),
            },
            
            Test::DnsLookup(_) => TestRequirements {
                node_types: None,
                node_capabilities: None,
                node_target_types: Some(vec![
                    NodeTarget::service_template(),
                    NodeTarget::hostname_template()
                ]),
                warning_message: Some("DnsLookup test requires a node with an hostname to validate resolution".to_string()),
            },
            
            Test::DnsOverHttps(_) => TestRequirements {
                node_types: Some(vec![NodeType::DnsServer]),
                node_capabilities: Some(vec![
                    NodeCapability::DnsService,
                ]),
                node_target_types: Some(vec![NodeTarget::service_template()]),
                warning_message: Some("DnsOverHttps test requires a DNS server with service target for HTTPS endpoint".to_string()),
            },
            
            Test::ReverseDns(_) => TestRequirements {
                node_types: None,
                node_capabilities: None,
                node_target_types: Some(vec![
                    NodeTarget::ipv4_template(),
                    NodeTarget::ipv6_template()
                ]),
                warning_message: Some("DnsLookup test requires a node with an IP to validate reverse DNS resolution".to_string()),
            },
            
            Test::VpnConnectivity(_) => TestRequirements {
                node_types: Some(vec![NodeType::VpnServer]),
                node_capabilities: Some(vec![NodeCapability::VpnService]),
                node_target_types: None,
                warning_message: Some("VpnConnectivity test should be assigned to VPN servers".to_string()),
            },
            
            Test::VpnTunnel(_) => TestRequirements {
                node_types: Some(vec![NodeType::VpnServer]),
                node_capabilities: Some(vec![NodeCapability::VpnService]),
                node_target_types: None,
                warning_message: Some("VpnTunnel test should be assigned to VPN servers".to_string()),
            },
            
            // Test::DaemonCommand(_) => TestRequirements {
            //     node_types: Some(vec![
            //         NodeType::Router,
            //         NodeType::WebServer,
            //         NodeType::DatabaseServer,
            //         NodeType::VpnServer,
            //         NodeType::NasDevice,
            //         NodeType::Workstation
            //     ]),
            //     node_capabilities: None,
            //     node_target_types: None,
            //     warning_message: "DaemonCommand test requires a node that can run NetFrog daemon (not suitable for IoT devices, printers, or cameras)".to_string(),
            // },
            
            // Test::SshScript(_) => TestRequirements {
            //     node_types: None,
            //     node_capabilities: Some(vec![NodeCapability::SshAccess]),
            //     node_target_types: None,
            //     warning_message: "SshScript test requires a node with SSH access capability".to_string(),
            // },
        }
    }

    pub fn check_node_compatibility(&self, node: &Node) -> Option<String> {
        let requirements = self.get_requirements();
        
        // Check node types if specified
        if let Some(required_types) = &requirements.node_types {
            if !required_types.contains(&node.base.node_type) {
                return requirements.warning_message.clone();
            }
        }
        
        // Check node capabilities if specified
        if let Some(required_capabilities) = &requirements.node_capabilities {
            let has_required_capability = required_capabilities.iter()
                .any(|cap| node.base.capabilities.contains(cap));
            
            if !has_required_capability {
                return requirements.warning_message.clone();
            }
        }
        
        // Check node target types if specified
        if let Some(required_target_types) = &requirements.node_target_types {
            let node_target_matches = required_target_types.iter().any(|required_target| {
                discriminant(&node.base.target) == discriminant(required_target)
            });
            
            if !node_target_matches {
                return requirements.warning_message.clone();
            }
        }
        
        // All requirements satisfied
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ConnectivityConfig {
    pub timeout_ms: Option<u32>,
    pub protocol: Option<TransportProtocol>,
}

impl Default for ConnectivityConfig {
    fn default() -> Self {
        Self {
            timeout_ms: Some(30000),
            protocol: Some(TransportProtocol::Tcp)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DirectIpConfig {
    pub timeout_ms: Option<u32>,
}

impl Default for DirectIpConfig {
    fn default() -> Self {
        Self {
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct PingConfig {
    pub packet_count: Option<u8>,          
    pub timeout_ms: Option<u32>,
}

impl Default for PingConfig {
    fn default() -> Self {
        Self {
            packet_count: Some(4),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ServiceHealthConfig {
    pub expected_status_code: u16,
    pub timeout_ms: Option<u32>,
}

impl Default for ServiceHealthConfig {
    fn default() -> Self {
        Self {
            expected_status_code: 200,
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DnsResolutionConfig {
    pub domain: String,                    // Domain to resolve
    pub expected_ip: IpAddr,         // Expected resolution results
    pub timeout_ms: Option<u32>,
}

impl Default for DnsResolutionConfig {
    fn default() -> Self {
        Self {
            domain: "example.com".to_string(),
            expected_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DnsOverHttpsConfig {
    pub domain: String,                    // Domain to resolve via DoH
    pub expected_ip: IpAddr,         // Expected resolution results
    pub timeout_ms: Option<u32>,
}

impl Default for DnsOverHttpsConfig {
    fn default() -> Self {
        Self {
            domain: "example.com".to_string(),
            expected_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DnsLookupConfig {
    pub expected_ip: IpAddr,
    pub timeout_ms: Option<u32>,
}

impl Default for DnsLookupConfig {
    fn default() -> Self {
        Self {
            expected_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ReverseDnsConfig {
    pub expected_domain: String,
    pub timeout_ms: Option<u32>,
}

impl Default for ReverseDnsConfig {
    fn default() -> Self {
        Self {
            expected_domain: "example.com".to_string(),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct VpnConnectivityConfig {
    pub timeout_ms: Option<u32>,
}

impl Default for VpnConnectivityConfig {
    fn default() -> Self {
        Self {
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct VpnTunnelConfig {
    pub expected_subnet: IpCidr,
    pub timeout_ms: Option<u32>,
}

impl Default for VpnTunnelConfig {
    fn default() -> Self {
        Self {
            expected_subnet: IpCidr::V4(Ipv4Cidr::new(
                Ipv4Addr::new(10, 100, 0, 0), 
                24
            ).expect("Default value should be valid IP")),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct NtpSyncConfig {
    pub max_offset_ms: u32,               // Maximum acceptable time offset
    pub timeout_ms: Option<u32>,
}

impl Default for NtpSyncConfig {
    fn default() -> Self {
        Self {
            timeout_ms: Some(30000),
            max_offset_ms: 1000
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
// pub struct DaemonCommandConfig {
//     pub command: String,                   // Command to execute
//     pub expected_output: Option<String>,   // Expected command output
//     pub requires_confirmation: Option<bool>, // User confirmation required
//     pub rollback_command: Option<String>,  // Rollback command if needed
//     pub timeout_ms: Option<u32>,
//     // Executes on the node via installed daemon
// }

// #[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
// pub struct SshScriptConfig {
//     pub command: String,                   // Command to execute via SSH
//     pub ssh_user: Option<String>,          // SSH username (default from node config)
//     pub expected_output: Option<String>,   // Expected command output
//     pub requires_confirmation: Option<bool>, // User confirmation required
//     pub rollback_command: Option<String>,  // Rollback command if needed
//     pub timeout_ms: Option<u32>,
//     // Executes on the node via SSH connection
// }

pub struct Timer {
    instant: Instant,
    datetime: DateTime<Utc>,
}

impl Timer {
    pub fn now() -> Self {
        Self {
            instant: Instant::now(),
            datetime: Utc::now(),
        }
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        self.instant.elapsed().as_millis() as u64
    }
    
    pub fn datetime(&self) -> DateTime<Utc> {
        self.datetime
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub success: bool,
    pub message: String,
    pub duration_ms: u64,
    pub executed_at: DateTime<Utc>,
    pub details: Option<serde_json::Value>, // Test-specific result data
    pub criticality: Option<TestCriticality>
}

impl TestResult {
    pub fn error_result(error: anyhow::Error, criticality: Option<TestCriticality>, timer: Timer) -> Self {
        Self {
            criticality: criticality,
            success: false,
            message: "Error executing test".to_string(),
            details: Some(serde_json::json!({
                "error": error.to_string(),
            })),
            duration_ms: timer.elapsed_ms(),
            executed_at: timer.datetime(),
        }
    }
}