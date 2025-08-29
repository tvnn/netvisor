use anyhow::Error;
use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;

use crate::server::{nodes::types::targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}, shared::types::{metadata::TypeMetadataProvider}};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityConfig {
    pub port: Option<u16>,    // Port where service was detected
    pub process: Option<String>, // Process name that matched  
    pub manual: bool,                   // True if user manually added
    pub system: bool                    // For daemon capability, assigned by system
}

impl CapabilityConfig {
    pub fn from_port(port: u16) -> Self {
        Self {
            port: Some(port),
            process: None,
            manual: false,
            system: false
        }
    }
    
    pub fn from_process(process_name: String) -> Self {
        Self {
            port: None,
            process: Some(process_name),
            manual: false,
            system: false
        }
    }
    
    pub fn manual() -> Self {
        Self {
            port: None,
            process: None,
            manual: true,
            system: false
        }
    }

    pub fn system(port: Option<u16>, process: Option<String>) -> Self {
        Self {
            port,
            process,
            manual: false,
            system: true
        }
    }
    
    pub fn set_port(&mut self, port: u16) {
        self.port = Some(port);
    }
    
    pub fn set_process(&mut self, process_name: String) {
        self.process = Some(process_name);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Serialize, Deserialize))]
pub enum NodeCapability {
    SshAccess { 
        config: CapabilityConfig,
        // Future: SSH keys, auth config, etc.
    },
    HttpService { 
        config: CapabilityConfig,
        path: Option<String>,
    },
    HttpsService { 
        config: CapabilityConfig,
        path: Option<String>,
        // Future: certificate info, auth config, etc.
    },
    DnsService { 
        config: CapabilityConfig,
        // Future: DNS server config, zones, etc.
    },
    WireGuardService { 
        config: CapabilityConfig,
        // Future: WireGuard config, peers, etc.
    },
    OpenVpnService { 
        config: CapabilityConfig,
        // Future: OpenVPN config, certificates, etc.
    },
    IpsecService { 
        config: CapabilityConfig,
        // Future: IPSec tunnels, certificates, etc.
    },
    SnmpService { 
        config: CapabilityConfig,
        // Future: SNMP communities, OIDs, etc.
    },
    TelnetService { 
        config: CapabilityConfig,
        // Future: access credentials, etc.
    },
    RdpService { 
        config: CapabilityConfig,
        // Future: RDP credentials, display config, etc.
    },
    DhcpService { 
        config: CapabilityConfig,
        // Future: RDP credentials, display config, etc.
    },
    DaemonService { 
        config: CapabilityConfig,
        daemon_id: Uuid,
    },
}

impl NodeCapability {

    pub fn build_http_endpoint(&self, target: &NodeTarget) -> Result<String, Error> {

        let port = match self.config().port {
            Some(p) => p,
            None => return Err(Error::msg("Selected capability does not have a port"))
        };

        let target = match target {
            NodeTarget::Hostname(HostnameTargetConfig{hostname}) => hostname.to_string(),
            NodeTarget::IpAddress(IpAddressTargetConfig{ip}) => ip.to_string()
        };

        match self {
            NodeCapability::HttpsService { path, .. } => {
                let path_str = path.as_deref().unwrap_or("/");
                Ok(format!("http://{}:{}{}", target, port, path_str))
            },
            NodeCapability::HttpService { path, .. } => {
                let path_str = path.as_deref().unwrap_or("/");
                Ok(format!("https://{}:{}{}", target, port, path_str))
            },
            NodeCapability::DaemonService { .. } => Ok(format!("https://{}:{}", target, port)),
            _ => Err(Error::msg("Selected capability does not support http endpoints"))
        }
    }

    pub fn config(&self) -> &CapabilityConfig {
        match self {
            NodeCapability::SshAccess{ config, .. } => config,
            NodeCapability::HttpService{ config, .. } => config,
            NodeCapability::HttpsService{ config, .. } => config,
            NodeCapability::WireGuardService{ config, .. } => config,
            NodeCapability::OpenVpnService{ config, .. } => config,
            NodeCapability::IpsecService{ config, .. } => config,
            NodeCapability::SnmpService{ config, .. } => config,
            NodeCapability::TelnetService{ config, .. } => config,
            NodeCapability::RdpService{ config, .. } => config,
            NodeCapability::DaemonService{ config, .. } => config,
            NodeCapability::DnsService{ config, .. } => config,
            NodeCapability::DhcpService{ config, .. } => config,
        }
    }
    pub fn config_mut(&mut self) -> &mut CapabilityConfig {
        match self {
            NodeCapability::SshAccess{ config, .. } => config,
            NodeCapability::HttpService{ config, .. } => config,
            NodeCapability::HttpsService{ config, .. } => config,
            NodeCapability::WireGuardService{ config, .. } => config,
            NodeCapability::OpenVpnService{ config, .. } => config,
            NodeCapability::IpsecService{ config, .. } => config,
            NodeCapability::SnmpService{ config, .. } => config,
            NodeCapability::TelnetService{ config, .. } => config,
            NodeCapability::RdpService{ config, .. } => config,
            NodeCapability::DaemonService{ config, .. } => config,
            NodeCapability::DnsService{ config, .. } => config,
            NodeCapability::DhcpService{ config, .. } => config,
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
            NodeCapability::DhcpService{ .. } => "DHCP Service",
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
            NodeCapability::DhcpService{ .. } => "Dynamic host Cconfiguration protocol service",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            NodeCapability::SshAccess{ .. } | NodeCapability::TelnetService{ .. } | NodeCapability::RdpService{ .. }  => "Remote Access",
            NodeCapability::HttpService{ .. } | NodeCapability::HttpsService{ .. } => "Web Services",
            NodeCapability::WireGuardService{ .. } | NodeCapability::OpenVpnService{ .. } | NodeCapability::IpsecService{ .. } => "Security",
            NodeCapability::DnsService{ .. } | NodeCapability::SnmpService{ .. } | NodeCapability::DhcpService{ .. } => "Network Infrastructure",
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
            NodeCapability::DhcpService{ .. } => "Router"
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeCapability::SshAccess{ .. } | NodeCapability::TelnetService{ .. } | NodeCapability::RdpService{ .. }  => "green",
            NodeCapability::HttpService{ .. } | NodeCapability::HttpsService{ .. } => "blue",
            NodeCapability::WireGuardService{ .. } | NodeCapability::OpenVpnService{ .. } | NodeCapability::IpsecService{ .. } => "orange",
            NodeCapability::DnsService{ .. } | NodeCapability::SnmpService{ .. } | NodeCapability::DhcpService{ .. } => "yellow",
            NodeCapability::DaemonService { .. } => "purple",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}