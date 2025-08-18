use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
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
}

// Capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
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
            _ => None,
        }
    }
}