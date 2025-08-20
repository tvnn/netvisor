use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::{shared::metadata::TypeMetadataProvider};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum NodeCapability {
    // Remote Access
    SshAccess,
    
    // Web Services  
    HttpService,
    HttpsService,
    
    // Network Infrastructure
    DnsService,
    DhcpService,
    
    // VPN Services
    VpnService,
    
    // Other Common Services
    FtpService,
    SmtpService,
    SnmpService,
}

impl NodeCapability {

    /// Get the default port associated with this capability (for auto-detection)
    pub fn default_port(&self) -> Option<u16> {
        match self {
            NodeCapability::SshAccess => Some(22),
            NodeCapability::HttpService => Some(80),
            NodeCapability::HttpsService => Some(443),
            NodeCapability::VpnService => Some(51820), // Wireguard default
            NodeCapability::DnsService => Some(53),
            NodeCapability::DhcpService => Some(67),
            _ => None
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

impl TypeMetadataProvider for NodeCapability {
    fn id(&self) -> String { 
        format!("{:?}", self) 
    }
    
    fn display_name(&self) -> &str {
        match self {
            NodeCapability::SshAccess => "SSH Access",
            NodeCapability::HttpService => "HTTP Service",
            NodeCapability::HttpsService => "HTTPS Service",
            NodeCapability::VpnService => "VPN Service",
            NodeCapability::DnsService => "DNS Service",
            NodeCapability::DhcpService => "DHCP Service",
            NodeCapability::FtpService => "FTP Service",
            NodeCapability::SmtpService => "SMTP Service",
            NodeCapability::SnmpService => "SNMP Service",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeCapability::SshAccess => "Remote command-line access for management and troubleshooting",
            NodeCapability::HttpService => "Web interface or API accessible over HTTP",
            NodeCapability::HttpsService => "Secure web interface or API accessible over HTTPS",
            NodeCapability::VpnService => "VPN server for secure remote access",
            NodeCapability::DnsService => "Domain name resolution service",
            NodeCapability::DhcpService => "Automatic IP address assignment for network devices",
            NodeCapability::FtpService => "File Transfer Protocol service for file sharing",
            NodeCapability::SmtpService => "Simple Mail Transfer Protocol for email delivery",
            NodeCapability::SnmpService => "Simple Network Management Protocol for device monitoring",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            NodeCapability::SshAccess => "Remote Access",
            NodeCapability::HttpService | NodeCapability::HttpsService => "Web Services",
            NodeCapability::VpnService => "Security",
            NodeCapability::DnsService | NodeCapability::DhcpService => "Network Infrastructure",
            NodeCapability::FtpService => "File Services",
            NodeCapability::SmtpService => "Communication",
            NodeCapability::SnmpService => "Management",

        }
    }
    
    fn icon(&self) -> &str {
        match self {
            NodeCapability::SshAccess => "Terminal",
            NodeCapability::HttpService | NodeCapability::HttpsService => "Globe",
            NodeCapability::VpnService => "Lock",
            NodeCapability::DnsService => "Search",
            NodeCapability::DhcpService => "Network",
            NodeCapability::FtpService => "Folder",
            NodeCapability::SmtpService => "Mail",
            NodeCapability::SnmpService => "Activity"
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeCapability::SshAccess | NodeCapability::FtpService => "green",
            NodeCapability::HttpService | NodeCapability::HttpsService | NodeCapability::SmtpService => "blue",
            NodeCapability::VpnService => "orange",
            NodeCapability::DnsService => "purple",
            NodeCapability::DhcpService => "cyan",
            NodeCapability::SnmpService => "yellow"
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}