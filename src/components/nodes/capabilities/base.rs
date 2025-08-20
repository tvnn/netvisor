use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};

use crate::{components::nodes::capabilities::{
    ssh::SshAccessCapability,
    http::*,
    dns::*,
    vpn::*,
    dhcp::*,
    ftp::*,
    smtp::*,
    snmp::*
}, 
shared::metadata::TypeMetadataProvider};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display))]
pub enum NodeCapability {
    // Remote Access
    SshAccess(SshAccessCapability),
    
    // Web Services  
    HttpService(HttpServiceCapability),
    HttpsService(HttpsServiceCapability),
    
    // Network Infrastructure
    DnsService(DnsServiceCapability),
    DhcpService(DhcpServiceCapability),
    
    // VPN Services
    VpnService(VpnServiceCapability),
    
    // Other Common Services
    FtpService(FtpServiceCapability),
    SmtpService(SmtpServiceCapability),
    SnmpService(SnmpServiceCapability),
}

impl NodeCapability {

    /// Get variant name as string
    pub fn variant_name(&self) -> String {
        NodeCapabilityDiscriminants::from(self).to_string()
    }

    /// Helper methods to access specific capability implementations
    pub fn as_dns_capability(&self) -> Option<&DnsServiceCapability> {
        match self {
            NodeCapability::DnsService(capability) => Some(capability),
            _ => None,
        }
    }

    /// Get the default port associated with this capability (for auto-detection)
    pub fn default_port(&self) -> Option<u16> {
        match self {
            NodeCapability::SshAccess(_) => Some(22),
            NodeCapability::HttpService(_) => Some(80),
            NodeCapability::HttpsService(_) => Some(443),
            NodeCapability::VpnService(_) => Some(51820), // Wireguard default
            NodeCapability::DnsService(_) => Some(53),
            NodeCapability::DhcpService(_) => Some(67),
            _ => None
        }
    }

    /// Create capability from discovered port (for auto-detection)
    pub fn from_port(port: u16) -> Option<Self> {
        match port {
            22 => Some(NodeCapability::SshAccess(SshAccessCapability {  })),
            80 => Some(NodeCapability::HttpService(HttpServiceCapability {  })),
            443 => Some(NodeCapability::HttpsService(HttpsServiceCapability {  })),
            53 => Some(NodeCapability::DnsService(DnsServiceCapability {  })),
            67 => Some(NodeCapability::DhcpService(DhcpServiceCapability {  })),
            1194 | 1723 | 500 | 4500 | 51820 => Some(NodeCapability::VpnService(VpnServiceCapability {  })),
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
            NodeCapability::SshAccess(_) => "SSH Access",
            NodeCapability::HttpService(_) => "HTTP Service",
            NodeCapability::HttpsService(_) => "HTTPS Service",
            NodeCapability::VpnService(_) => "VPN Service",
            NodeCapability::DnsService(_) => "DNS Service",
            NodeCapability::DhcpService(_) => "DHCP Service",
            NodeCapability::FtpService(_) => "FTP Service",
            NodeCapability::SmtpService(_) => "SMTP Service",
            NodeCapability::SnmpService(_) => "SNMP Service",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "Remote command-line access for management and troubleshooting",
            NodeCapability::HttpService(_) => "Web interface or API accessible over HTTP",
            NodeCapability::HttpsService(_) => "Secure web interface or API accessible over HTTPS",
            NodeCapability::VpnService(_) => "VPN server for secure remote access",
            NodeCapability::DnsService(_) => "Domain name resolution service",
            NodeCapability::DhcpService(_) => "Automatic IP address assignment for network devices",
            NodeCapability::FtpService(_) => "File Transfer Protocol service for file sharing",
            NodeCapability::SmtpService(_) => "Simple Mail Transfer Protocol for email delivery",
            NodeCapability::SnmpService(_) => "Simple Network Management Protocol for device monitoring",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "Remote Access",
            NodeCapability::HttpService(_) | NodeCapability::HttpsService(_) => "Web Services",
            NodeCapability::VpnService(_) => "Security",
            NodeCapability::DnsService(_) | NodeCapability::DhcpService(_) => "Network Infrastructure",
            NodeCapability::FtpService(_) => "File Services",
            NodeCapability::SmtpService(_) => "Communication",
            NodeCapability::SnmpService(_) => "Management",

        }
    }
    
    fn icon(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "Terminal",
            NodeCapability::HttpService(_) | NodeCapability::HttpsService(_) => "Globe",
            NodeCapability::VpnService(_) => "Lock",
            NodeCapability::DnsService(_) => "Search",
            NodeCapability::DhcpService(_) => "Network",
            NodeCapability::FtpService(_) => "Folder",
            NodeCapability::SmtpService(_) => "Mail",
            NodeCapability::SnmpService(_) => "Activity"
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) | NodeCapability::FtpService(_) => "green",
            NodeCapability::HttpService(_) | NodeCapability::HttpsService(_) | NodeCapability::SmtpService(_) => "blue",
            NodeCapability::VpnService(_) => "orange",
            NodeCapability::DnsService(_) => "purple",
            NodeCapability::DhcpService(_) => "cyan",
            NodeCapability::SnmpService(_) => "yellow"
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}