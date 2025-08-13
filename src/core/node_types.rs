use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    // Infrastructure (network-focused)
    Router,
    Switch,
    AccessPoint,
    Firewall,
    
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
    Printer,
    Camera,
    
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
    
    // Database Services
    MysqlService,
    PostgresService,
    MongoService,
    RedisService,
    
    // Other Services
    DnsService,
    EmailService,
    FtpService,
    
    // Custom
    CustomService(String, u16), // Named service on specific port
}

impl NodeType {
    /// Get the display name for this node type
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

    /// Get the category for this node type
    pub fn category(&self) -> &'static str {
        match self {
            NodeType::Router | NodeType::Switch | NodeType::AccessPoint | NodeType::Firewall => "Infrastructure",
            NodeType::WebServer | NodeType::DatabaseServer | NodeType::MediaServer | 
            NodeType::DnsServer | NodeType::VpnServer | NodeType::NasDevice => "Servers",
            NodeType::Workstation | NodeType::IotDevice | NodeType::Printer | NodeType::Camera => "Endpoints",
            NodeType::UnknownDevice => "Generic",
        }
    }

    /// Determine node type from discovered open ports (priority-based)
    pub fn from_open_ports(ports: &[u16]) -> Self {
        // Priority order: More specific service types first
        
        // VPN Server indicators
        if ports.contains(&51820) || ports.contains(&1194) {
            return NodeType::VpnServer;
        }
        
        // DNS Server indicators
        if ports.contains(&53) {
            return NodeType::DnsServer;
        }
        
        // Database Server indicators
        if ports.contains(&3306) || ports.contains(&5432) || ports.contains(&27017) || ports.contains(&6379) {
            return NodeType::DatabaseServer;
        }
        
        // Web Server indicators
        if ports.contains(&80) || ports.contains(&443) || ports.contains(&8080) || ports.contains(&8443) {
            return NodeType::WebServer;
        }
        
        // Media Server indicators
        if ports.contains(&32400) || ports.contains(&8096) || ports.contains(&8920) {
            return NodeType::MediaServer;
        }
        
        // NAS Device indicators
        if ports.contains(&445) || ports.contains(&139) || ports.contains(&2049) {
            return NodeType::NasDevice;
        }
        
        // Infrastructure indicators
        if ports.contains(&161) || ports.contains(&162) {
            // SNMP often indicates network infrastructure
            return NodeType::Router; // Default to router for SNMP
        }
        
        // Printer indicators
        if ports.contains(&631) || ports.contains(&9100) {
            return NodeType::Printer;
        }
        
        // Workstation indicators
        if ports.contains(&22) || ports.contains(&3389) || ports.contains(&5900) {
            return NodeType::Workstation;
        }
        
        // Default to unknown if we can't determine
        NodeType::UnknownDevice
    }

    /// Get default capabilities for this node type
    pub fn default_capabilities(&self) -> Vec<NodeCapability> {
        match self {
            NodeType::Router | NodeType::Switch | NodeType::AccessPoint | NodeType::Firewall => {
                vec![NodeCapability::SshAccess]
            },
            NodeType::WebServer => {
                vec![NodeCapability::HttpService, NodeCapability::HttpsService, NodeCapability::SshAccess]
            },
            NodeType::DatabaseServer => {
                vec![NodeCapability::SshAccess, NodeCapability::MysqlService] // Could be enhanced with detection
            },
            NodeType::MediaServer => {
                vec![NodeCapability::HttpService, NodeCapability::HttpsService, NodeCapability::SshAccess]
            },
            NodeType::DnsServer => {
                vec![NodeCapability::DnsService, NodeCapability::SshAccess]
            },
            NodeType::VpnServer => {
                vec![NodeCapability::SshAccess]
            },
            NodeType::NasDevice => {
                vec![NodeCapability::SshAccess, NodeCapability::HttpService]
            },
            NodeType::Workstation => {
                vec![NodeCapability::SshAccess, NodeCapability::RdpAccess]
            },
            NodeType::IotDevice => {
                vec![NodeCapability::HttpService]
            },
            NodeType::Printer => {
                vec![]
            },
            NodeType::Camera => {
                vec![NodeCapability::HttpService]
            },
            NodeType::UnknownDevice => {
                vec![]
            },
        }
    }
}

impl NodeCapability {
    /// Get the display name for this capability
    pub fn display_name(&self) -> String {
        match self {
            NodeCapability::SshAccess => "SSH Access".to_string(),
            NodeCapability::RdpAccess => "RDP Access".to_string(),
            NodeCapability::VncAccess => "VNC Access".to_string(),
            NodeCapability::HttpService => "HTTP Service".to_string(),
            NodeCapability::HttpsService => "HTTPS Service".to_string(),
            NodeCapability::MysqlService => "MySQL Service".to_string(),
            NodeCapability::PostgresService => "PostgreSQL Service".to_string(),
            NodeCapability::MongoService => "MongoDB Service".to_string(),
            NodeCapability::RedisService => "Redis Service".to_string(),
            NodeCapability::DnsService => "DNS Service".to_string(),
            NodeCapability::EmailService => "Email Service".to_string(),
            NodeCapability::FtpService => "FTP Service".to_string(),
            NodeCapability::CustomService(name, port) => format!("{} (Port {})", name, port),
        }
    }

    /// Get the default port for this capability (if applicable)
    pub fn default_port(&self) -> Option<u16> {
        match self {
            NodeCapability::SshAccess => Some(22),
            NodeCapability::RdpAccess => Some(3389),
            NodeCapability::VncAccess => Some(5900),
            NodeCapability::HttpService => Some(80),
            NodeCapability::HttpsService => Some(443),
            NodeCapability::MysqlService => Some(3306),
            NodeCapability::PostgresService => Some(5432),
            NodeCapability::MongoService => Some(27017),
            NodeCapability::RedisService => Some(6379),
            NodeCapability::DnsService => Some(53),
            NodeCapability::EmailService => Some(25),
            NodeCapability::FtpService => Some(21),
            NodeCapability::CustomService(_, port) => Some(*port),
        }
    }

    /// Detect capabilities from open ports
    pub fn from_open_ports(ports: &[u16]) -> Vec<Self> {
        let mut capabilities = Vec::new();
        
        for &port in ports {
            let capability = match port {
                22 => Some(NodeCapability::SshAccess),
                3389 => Some(NodeCapability::RdpAccess),
                5900 => Some(NodeCapability::VncAccess),
                80 | 8080 => Some(NodeCapability::HttpService),
                443 | 8443 => Some(NodeCapability::HttpsService),
                3306 => Some(NodeCapability::MysqlService),
                5432 => Some(NodeCapability::PostgresService),
                27017 => Some(NodeCapability::MongoService),
                6379 => Some(NodeCapability::RedisService),
                53 => Some(NodeCapability::DnsService),
                25 | 587 | 465 => Some(NodeCapability::EmailService),
                21 => Some(NodeCapability::FtpService),
                _ => None,
            };
            
            if let Some(cap) = capability {
                if !capabilities.contains(&cap) {
                    capabilities.push(cap);
                }
            }
        }
        
        capabilities
    }
}