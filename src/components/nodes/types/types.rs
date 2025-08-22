use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use crate::{components::nodes::capabilities::{base::{NodeCapabilityDiscriminants}}, shared::metadata::TypeMetadataProvider};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum NodeType {
    // Infrastructure (network-focused)
    Router, Switch, AccessPoint, Firewall,
    
    // Servers
    WebServer,
    DatabaseServer,
    MediaServer,
    DnsServer,
    VpnServer,
    NasDevice,
    
    // Endpoints
    Workstation,
    IotDevice,
    Printer, 
    Camera,
    
    // Generic
    UnknownDevice,  // Cannot determine primary function
}

impl NodeType {
    /// Get typical capabilities for this node type (for auto-assignment)
    pub fn typical_capabilities(&self) -> Vec<NodeCapabilityDiscriminants> {
        match self {
            NodeType::VpnServer => vec![
                NodeCapabilityDiscriminants::VpnService,
                NodeCapabilityDiscriminants::SshAccess,
                NodeCapabilityDiscriminants::HttpService,
            ],
            NodeType::Router => vec![
                NodeCapabilityDiscriminants::HttpService,
                NodeCapabilityDiscriminants::SshAccess,
                // NodeCapabilityDiscriminants::DhcpService,
            ],
            NodeType::Switch => vec![
                NodeCapabilityDiscriminants::HttpService,
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::AccessPoint => vec![
                NodeCapabilityDiscriminants::HttpService,
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::Firewall => vec![
                NodeCapabilityDiscriminants::HttpService,
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::DnsServer => vec![
                NodeCapabilityDiscriminants::DnsService,
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::WebServer => vec![
                NodeCapabilityDiscriminants::HttpService,
                NodeCapabilityDiscriminants::HttpsService,
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::DatabaseServer => vec![
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::MediaServer => vec![
                NodeCapabilityDiscriminants::HttpService,
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::NasDevice => vec![
                NodeCapabilityDiscriminants::SshAccess,
                NodeCapabilityDiscriminants::HttpService,
            ],
            NodeType::Workstation => vec![
                NodeCapabilityDiscriminants::SshAccess,
            ],
            NodeType::IotDevice => vec![
                NodeCapabilityDiscriminants::HttpService,
            ],
            NodeType::Printer => vec![
                NodeCapabilityDiscriminants::HttpService,
            ],
            NodeType::Camera => vec![
                NodeCapabilityDiscriminants::HttpService,
            ],
            NodeType::UnknownDevice => vec![],
        }
    }
}

impl TypeMetadataProvider for NodeType {
    fn id(&self) -> String { 
        format!("{:?}", self)
    }
    
    fn display_name(&self) -> &'static str {
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
    
    fn description(&self) -> &str {
        match self {
            NodeType::Router => "Network router for traffic routing and gateway functions",
            NodeType::Switch => "Network switch for device connectivity within VLANs",
            NodeType::AccessPoint => "Wireless access point for WiFi connectivity",
            NodeType::Firewall => "Security appliance for network traffic filtering",
            NodeType::VpnServer => "VPN server for secure remote access",
            NodeType::DnsServer => "DNS server for domain name resolution",
            NodeType::WebServer => "Web server hosting websites and web applications",
            NodeType::DatabaseServer => "Database server for data storage and management",
            NodeType::MediaServer => "Media server for streaming video and audio content",
            NodeType::NasDevice => "Network attached storage for file sharing",
            NodeType::Workstation => "Desktop computer or laptop for user productivity",
            NodeType::IotDevice => "Internet of Things device with network connectivity",
            NodeType::Printer => "Network printer for document printing",
            NodeType::Camera => "IP camera for security or monitoring",
            NodeType::UnknownDevice => "Device with undetermined primary function",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            NodeType::Router | NodeType::Switch | NodeType::AccessPoint | NodeType::Firewall => "Infrastructure",
            NodeType::VpnServer | NodeType::DnsServer | NodeType::WebServer | NodeType::DatabaseServer | NodeType::MediaServer => "Server",
            NodeType::NasDevice => "Storage",
            NodeType::Workstation | NodeType::IotDevice | NodeType::Printer | NodeType::Camera => "Endpoint",
            NodeType::UnknownDevice => "Unknown",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            NodeType::Router => "Router",
            NodeType::Switch => "Network",
            NodeType::AccessPoint => "Wifi",
            NodeType::Firewall => "Shield",
            NodeType::VpnServer => "Lock",
            NodeType::DnsServer => "Search",
            NodeType::WebServer => "Globe",
            NodeType::DatabaseServer => "Database",
            NodeType::MediaServer => "Play",
            NodeType::NasDevice => "HardDrive",
            NodeType::Workstation => "Monitor",
            NodeType::IotDevice => "Cpu",
            NodeType::Printer => "Printer",
            NodeType::Camera => "Camera",
            NodeType::UnknownDevice => "HelpCircle",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeType::Router | NodeType::Switch | NodeType::AccessPoint => "blue",
            NodeType::Firewall => "red",
            NodeType::VpnServer => "orange",
            NodeType::DnsServer => "purple",
            NodeType::WebServer => "green",
            NodeType::DatabaseServer => "yellow",
            NodeType::MediaServer => "pink",
            NodeType::NasDevice | NodeType::Workstation | NodeType::Printer | NodeType::Camera => "gray",
            NodeType::IotDevice => "teal",
            NodeType::UnknownDevice => "gray",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({
            "typical_capabilities": self.typical_capabilities(),
        })
    }
}