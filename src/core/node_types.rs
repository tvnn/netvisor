// src/core/node_types.rs - Simplified capabilities
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeCapability {
    // Remote Access
    SshAccess, 
    RdpAccess, 
    VncAccess,
    
    // Web Services  
    HttpService, 
    HttpsService,
    
    // Simplified Database Service (combines all database types)
    DatabaseService,
    
    // Other Services
    DnsService, 
    EmailService, 
    FtpService,
    
    // Custom
    CustomService(String, u16), // Named service on specific port
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

    /// Get suggested capabilities for this node type
    pub fn suggested_capabilities(&self) -> Vec<NodeCapability> {
        match self {
            NodeType::WebServer => vec![NodeCapability::HttpService, NodeCapability::HttpsService, NodeCapability::SshAccess],
            NodeType::DatabaseServer => vec![NodeCapability::DatabaseService, NodeCapability::SshAccess],
            NodeType::DnsServer => vec![NodeCapability::DnsService, NodeCapability::SshAccess],
            NodeType::VpnServer => vec![NodeCapability::SshAccess],
            NodeType::NasDevice => vec![NodeCapability::SshAccess, NodeCapability::HttpService],
            NodeType::MediaServer => vec![NodeCapability::HttpService, NodeCapability::SshAccess],
            NodeType::Router | NodeType::Switch | NodeType::AccessPoint => vec![NodeCapability::HttpService, NodeCapability::SshAccess],
            NodeType::Firewall => vec![NodeCapability::HttpService, NodeCapability::SshAccess],
            NodeType::Workstation => vec![NodeCapability::SshAccess, NodeCapability::RdpAccess, NodeCapability::VncAccess],
            NodeType::Printer => vec![NodeCapability::HttpService],
            NodeType::Camera => vec![NodeCapability::HttpService],
            NodeType::IotDevice => vec![],
            NodeType::UnknownDevice => vec![],
        }
    }
}

impl NodeCapability {
    pub fn display_name(&self) -> String {
        match self {
            NodeCapability::SshAccess => "SSH Access".to_string(),
            NodeCapability::RdpAccess => "RDP Access".to_string(),
            NodeCapability::VncAccess => "VNC Access".to_string(),
            NodeCapability::HttpService => "HTTP Service".to_string(),
            NodeCapability::HttpsService => "HTTPS Service".to_string(),
            NodeCapability::DatabaseService => "Database Service".to_string(),
            NodeCapability::DnsService => "DNS Service".to_string(),
            NodeCapability::EmailService => "Email Service".to_string(),
            NodeCapability::FtpService => "FTP Service".to_string(),
            NodeCapability::CustomService(name, port) => format!("{} (Port {})", name, port),
        }
    }

    /// Get all available capabilities for UI selection
    pub fn all_capabilities() -> Vec<NodeCapability> {
        vec![
            NodeCapability::SshAccess,
            NodeCapability::RdpAccess,
            NodeCapability::VncAccess,
            NodeCapability::HttpService,
            NodeCapability::HttpsService,
            NodeCapability::DatabaseService,
            NodeCapability::DnsService,
            NodeCapability::EmailService,
            NodeCapability::FtpService,
        ]
    }
}