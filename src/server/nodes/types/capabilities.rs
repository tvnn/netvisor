use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;

use crate::{server::shared::types::metadata::TypeMetadataProvider};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilitySource {
    pub port_detection: Option<u16>,    // Port where service was detected
    pub process_detection: Option<String>, // Process name that matched  
    pub manual: bool,                   // True if user manually added
}

impl CapabilitySource {
    pub fn from_port(port: u16) -> Self {
        Self {
            port_detection: Some(port),
            process_detection: None,
            manual: false,
        }
    }
    
    pub fn from_process(process_name: String) -> Self {
        Self {
            port_detection: None,
            process_detection: Some(process_name),
            manual: false,
        }
    }
    
    pub fn manual() -> Self {
        Self {
            port_detection: None,
            process_detection: None,
            manual: true,
        }
    }
    
    pub fn set_port(&mut self, port: u16) {
        self.port_detection = Some(port);
    }
    
    pub fn set_process(&mut self, process_name: String) {
        self.process_detection = Some(process_name);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Serialize, Deserialize))]
pub enum NodeCapability {
    SshAccess { 
        source: CapabilitySource,
        // Future: SSH keys, auth config, etc.
    },
    HttpService { 
        source: CapabilitySource,
        // Future: authentication, SSL config, etc.
    },
    HttpsService { 
        source: CapabilitySource,
        // Future: certificate info, auth config, etc.
    },
    DnsService { 
        source: CapabilitySource,
        // Future: DNS server config, zones, etc.
    },
    WireGuardService { 
        source: CapabilitySource,
        // Future: WireGuard config, peers, etc.
    },
    OpenVpnService { 
        source: CapabilitySource,
        // Future: OpenVPN config, certificates, etc.
    },
    IpsecService { 
        source: CapabilitySource,
        // Future: IPSec tunnels, certificates, etc.
    },
    SnmpService { 
        source: CapabilitySource,
        // Future: SNMP communities, OIDs, etc.
    },
    TelnetService { 
        source: CapabilitySource,
        // Future: access credentials, etc.
    },
    RdpService { 
        source: CapabilitySource,
        // Future: RDP credentials, display config, etc.
    },
    DaemonService { 
        source: CapabilitySource,
        daemon_id: Uuid,
    },
}

impl NodeCapability {
    pub fn source(&self) -> &CapabilitySource {
        match self {
            NodeCapability::SshAccess{ source, .. } => source,
            NodeCapability::HttpService{ source, .. } => source,
            NodeCapability::HttpsService{ source, .. } => source,
            NodeCapability::WireGuardService{ source, .. } => source,
            NodeCapability::OpenVpnService{ source, .. } => source,
            NodeCapability::IpsecService{ source, .. } => source,
            NodeCapability::SnmpService{ source, .. } => source,
            NodeCapability::TelnetService{ source, .. } => source,
            NodeCapability::RdpService{ source, .. } => source,
            NodeCapability::DaemonService{ source, .. } => source,
            NodeCapability::DnsService{ source, .. } => source,
        }
    }
    pub fn source_mut(&mut self) -> &mut CapabilitySource {
        match self {
            NodeCapability::SshAccess{ source, .. } => source,
            NodeCapability::HttpService{ source, .. } => source,
            NodeCapability::HttpsService{ source, .. } => source,
            NodeCapability::WireGuardService{ source, .. } => source,
            NodeCapability::OpenVpnService{ source, .. } => source,
            NodeCapability::IpsecService{ source, .. } => source,
            NodeCapability::SnmpService{ source, .. } => source,
            NodeCapability::TelnetService{ source, .. } => source,
            NodeCapability::RdpService{ source, .. } => source,
            NodeCapability::DaemonService{ source, .. } => source,
            NodeCapability::DnsService{ source, .. } => source,
        }
    }
}

impl TypeMetadataProvider for NodeCapability {
    fn id(&self) -> String { 
        self.discriminant().to_string()
    } 

    fn display_name(&self) -> &str {
        match self {
            NodeCapability::SshAccess{ .. } => "SSH Access",
            NodeCapability::HttpService{ .. } => "HTTP Service",
            NodeCapability::HttpsService{ .. } => "HTTPS Service",
            NodeCapability::WireGuardService{ .. } => "Wireguard VPN Service",
            NodeCapability::OpenVpnService{ .. } => "OpenVPN Service",
            NodeCapability::IpsecService{ .. } => "IPSec VPN Service",
            NodeCapability::SnmpService{ .. } => "SNMP Service",
            NodeCapability::TelnetService{ .. } => "Telnet Service",
            NodeCapability::RdpService{ .. } => "RDP Service",
            NodeCapability::DaemonService{ .. } => "NetVisor Daemon Service",
            NodeCapability::DnsService{ .. } => "DNS Service",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeCapability::SshAccess { .. } => "Remote command-line access for management and troubleshooting",
            NodeCapability::HttpService { .. } => "Web service providing HTTP content",
            NodeCapability::HttpsService { .. } => "Secure web service providing HTTPS content", 
            NodeCapability::DnsService { .. } => "Domain name resolution service",
            NodeCapability::WireGuardService { .. } => "Modern VPN service using WireGuard protocol",
            NodeCapability::OpenVpnService { .. } => "VPN service using OpenVPN protocol",
            NodeCapability::IpsecService { .. } => "VPN service using IPSec protocol",
            NodeCapability::SnmpService { .. } => "Network device monitoring and management",
            NodeCapability::TelnetService { .. } => "Remote terminal access (legacy)",
            NodeCapability::RdpService { .. } => "Remote desktop access for Windows systems",
            NodeCapability::DaemonService { .. } => "NetVisor daemon for enhanced network diagnostics",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            NodeCapability::SshAccess{ .. } | NodeCapability::TelnetService{ .. } | NodeCapability::RdpService{ .. }  => "Remote Access",
            NodeCapability::HttpService{ .. } | NodeCapability::HttpsService{ .. } => "Web Services",
            NodeCapability::WireGuardService{ .. } | NodeCapability::OpenVpnService{ .. } | NodeCapability::IpsecService{ .. } => "Security",
            NodeCapability::DnsService{ .. } | NodeCapability::SnmpService{ .. } => "Network Infrastructure",
            NodeCapability::DaemonService { .. } => "NetVisor",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            NodeCapability::SshAccess{ .. } | NodeCapability::TelnetService{ .. } | NodeCapability::RdpService{ .. }  => "Terminal",
            NodeCapability::HttpService{ .. } | NodeCapability::HttpsService{ .. } => "Globe",
            NodeCapability::WireGuardService{ .. } | NodeCapability::OpenVpnService{ .. } | NodeCapability::IpsecService{ .. } => "Lock",
            NodeCapability::DnsService{ .. } | NodeCapability::SnmpService{ .. } => "Search",
            NodeCapability::DaemonService { .. } => "RectangleGoggles",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeCapability::SshAccess{ .. } | NodeCapability::TelnetService{ .. } | NodeCapability::RdpService{ .. }  => "green",
            NodeCapability::HttpService{ .. } | NodeCapability::HttpsService{ .. } => "blue",
            NodeCapability::WireGuardService{ .. } | NodeCapability::OpenVpnService{ .. } | NodeCapability::IpsecService{ .. } => "orange",
            NodeCapability::DnsService{ .. } | NodeCapability::SnmpService{ .. } => "yellow",
            NodeCapability::DaemonService { .. } => "purple",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}