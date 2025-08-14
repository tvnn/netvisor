use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::components::tests::types::{TestType, TestConfiguration, TestResult};

// API Requests and Responses
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeRequest {
    #[serde(flatten)]
    pub node: NodeBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeRequest {
    pub name: Option<String>,
    pub domain: Option<Option<String>>,
    pub ip: Option<Option<String>>,
    pub port: Option<Option<u16>>,
    pub path: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub node_type: Option<Option<NodeType>>,
    pub capabilities: Option<Vec<NodeCapability>>,
    pub monitoring_enabled: Option<bool>,
    pub assigned_tests: Option<Vec<AssignedTest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignTestRequest {
    pub node_id: String,
    pub test_type: TestType,
    pub test_config: TestConfiguration,
    pub criticality: TestCriticality,
    pub monitor_interval_minutes: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub node: Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListResponse {
    pub nodes: Vec<Node>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeCompatibilityResponse {
    pub node_id: String,
    pub compatible_test_types: Vec<TestType>,
    pub incompatible_test_types: Vec<TestType>,
    pub missing_capabilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTestResultsResponse {
    pub node_id: String,
    pub results: Vec<TestResult>,
    pub executed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentFailure {
    pub node_id: String,
    pub node_name: String,
    pub test_type: TestType,
    pub failed_at: String,
    pub error_message: String,
}

// Base and entity
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

// Current health state of a node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Healthy,    // All checks passing
    Degraded,   // Some checks failing but node functional
    Failed,     // Critical checks failing
    Unknown,    // No recent check data
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

// Node test relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedTest {
    pub test_type: TestType,
    pub test_config: TestConfiguration,
    pub monitor_interval_minutes: Option<u32>,  // None = diagnostic-only
    pub enabled: bool,
    pub criticality: TestCriticality,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TestCriticality {
    Critical,        // Failure = NodeStatus::Failed
    Important,       // Failure = NodeStatus::Degraded  
    Informational,   // Failure = NodeStatus::Healthy (just logged)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTestResults {
    pub node_id: String,
    pub test_results: Vec<TestResult>,
    pub node_status: NodeStatus,  // Computed from test results + criticality
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

// Graph positioning for topology visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPosition {
    pub x: f32,
    pub y: f32,
    pub z: Option<f32>, // For 3D layouts if needed
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    // Infrastructure (network-focused)
    Router, Switch, AccessPoint, Firewall,
    
    // Servers (service-focused, primary service wins)
    WebServer,      // Primary: HTTP/HTTPS
    DatabaseServer, // Primary: Database service
    MediaServer,    // Primary: Media streaming
    DnsServer,      // Primary: DNS service
    VpnServer,      // Primary: VPN service
    NasDevice,      // Primary: File storage
    
    // Endpoints
    Workstation,    // General computer
    IotDevice,      // Smart home device
    Printer, Camera,
    
    // Generic
    UnknownDevice,  // Cannot determine primary function
}

impl NodeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeType::Router => "Router",
            NodeType::Switch => "Switch",
            NodeType::AccessPoint => "Access Point",
            NodeType::Firewall => "Firewall",
            NodeType::WebServer => "Web Server",
            NodeType::DatabaseServer => "Database Server",
            NodeType::MediaServer => "Media Server",
            NodeType::DnsServer => "DNS Server",
            NodeType::VpnServer => "VPN Server",
            NodeType::NasDevice => "NAS Device",
            NodeType::Workstation => "Workstation",
            NodeType::IotDevice => "IoT Device",
            NodeType::Printer => "Printer",
            NodeType::Camera => "Camera",
            NodeType::UnknownDevice => "Unknown Device",
        }
    }

    /// Get typical capabilities for this node type (for auto-assignment)
    pub fn typical_capabilities(&self) -> Vec<NodeCapability> {
        match self {
            NodeType::VpnServer => vec![
                NodeCapability::VpnService,
                NodeCapability::SshAccess,
                NodeCapability::HttpService,
            ],
            NodeType::Router => vec![
                NodeCapability::HttpService,
                NodeCapability::SshAccess,
                NodeCapability::DhcpService,
            ],
            NodeType::Switch => vec![
                NodeCapability::HttpService,
                NodeCapability::SshAccess,
            ],
            NodeType::AccessPoint => vec![
                NodeCapability::HttpService,
                NodeCapability::SshAccess,
            ],
            NodeType::Firewall => vec![
                NodeCapability::HttpService,
                NodeCapability::SshAccess,
            ],
            NodeType::DnsServer => vec![
                NodeCapability::DnsService,
                NodeCapability::SshAccess,
            ],
            NodeType::WebServer => vec![
                NodeCapability::HttpService,
                NodeCapability::HttpsService,
                NodeCapability::SshAccess,
            ],
            NodeType::DatabaseServer => vec![
                NodeCapability::SshAccess,
            ],
            NodeType::MediaServer => vec![
                NodeCapability::HttpService,
                NodeCapability::SshAccess,
            ],
            NodeType::NasDevice => vec![
                NodeCapability::SshAccess,
                NodeCapability::HttpService,
            ],
            NodeType::Workstation => vec![
                NodeCapability::SshAccess,
            ],
            NodeType::IotDevice => vec![
                NodeCapability::HttpService,
            ],
            NodeType::Printer => vec![
                NodeCapability::HttpService,
            ],
            NodeType::Camera => vec![
                NodeCapability::HttpService,
            ],
            NodeType::UnknownDevice => vec![],
        }
    }

    /// Auto-detect capabilities from open ports (for discovery)
    pub fn detect_from_open_ports(open_ports: &[u16]) -> Self {
        let capabilities: Vec<NodeCapability> = open_ports
            .iter()
            .filter_map(|&port| NodeCapability::from_port(port))
            .collect();

        // Priority-based detection for VPN use case
        if capabilities.contains(&NodeCapability::VpnService) {
            return NodeType::VpnServer;
        }

        if capabilities.contains(&NodeCapability::DnsService) {
            return NodeType::DnsServer;
        }

        // Check for router indicators
        if capabilities.contains(&NodeCapability::DhcpService) {
            return NodeType::Router;
        }

        // Web server if both HTTP and HTTPS
        if capabilities.contains(&NodeCapability::HttpService) 
            && capabilities.contains(&NodeCapability::HttpsService) {
            return NodeType::WebServer;
        }

        // Default fallback
        NodeType::UnknownDevice
    }
}

// Capabilities

#[derive(Debug, Serialize)]
pub struct CapabilityRecommendations {
    pub all_capabilities: Vec<NodeCapability>,
    pub current_capabilities: Vec<NodeCapability>, 
    pub suggested_capabilities: Vec<NodeCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeCapability {
    // Remote Access (core for VPN troubleshooting)
    SshAccess,
    HttpService,
    HttpsService,
    
    // VPN-specific
    VpnService,
    
    // Network Infrastructure  
    DnsService,
    DhcpService,
    
    // Custom service detection
    CustomService(String, u16), // service_name, port
}

impl NodeCapability {
    pub fn display_name(&self) -> String {
        match self {
            NodeCapability::SshAccess => "SSH Access".to_string(),
            NodeCapability::HttpService => "HTTP Service".to_string(),
            NodeCapability::HttpsService => "HTTPS Service".to_string(),
            NodeCapability::VpnService => "VPN Service".to_string(),
            NodeCapability::DnsService => "DNS Service".to_string(),
            NodeCapability::DhcpService => "DHCP Service".to_string(),
            NodeCapability::CustomService(name, port) => format!("{} (Port {})", name, port),
        }
    }

    /// Get the default port associated with this capability (for auto-detection)
    pub fn default_port(&self) -> Option<u16> {
        match self {
            NodeCapability::SshAccess => Some(22),
            NodeCapability::HttpService => Some(80),
            NodeCapability::HttpsService => Some(443),
            NodeCapability::VpnService => Some(51820), // Wireguard default
            NodeCapability::DnsService => Some(53),
            NodeCapability::DhcpService => Some(67),
            NodeCapability::CustomService(_, port) => Some(*port),
        }
    }

    /// Create capability from discovered port (for auto-detection)
    pub fn from_port(port: u16) -> Option<Self> {
        match port {
            22 => Some(NodeCapability::SshAccess),
            80 => Some(NodeCapability::HttpService),
            443 => Some(NodeCapability::HttpsService),
            53 => Some(NodeCapability::DnsService),
            67 => Some(NodeCapability::DhcpService),
            1194 | 1723 | 500 | 4500 | 51820 => Some(NodeCapability::VpnService),
            _ => None, // Will be handled as CustomService if needed
        }
    }

    pub fn all() -> Vec<Self> {  // CHANGE: Remove the semicolon, this should return Vec<Self>
        vec![
            NodeCapability::SshAccess,
            NodeCapability::HttpService,
            NodeCapability::HttpsService,
            NodeCapability::VpnService,
            NodeCapability::DnsService,
            NodeCapability::DhcpService,
        ]
    }

    /// Get all standard capabilities (excluding CustomService)
    pub fn all_standard() -> Vec<NodeCapability> {
        vec![
            NodeCapability::SshAccess,
            NodeCapability::HttpService,
            NodeCapability::HttpsService,
            NodeCapability::VpnService,
            NodeCapability::DnsService,
            NodeCapability::DhcpService,
        ]
    }
}