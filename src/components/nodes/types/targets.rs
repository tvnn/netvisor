use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use strum_macros::{EnumIter, EnumDiscriminants, Display};

use crate::shared::types::{metadata::TypeMetadataProvider, protocols::ApplicationProtocol};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum NodeTarget {
    IpAddress(IpAddressTargetConfig),
    Hostname(HostnameTargetConfig),
    Service(ServiceTargetConfig)
}

impl std::fmt::Display for NodeTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeTarget::IpAddress(config) => write!(f, "{}", config),
            NodeTarget::Hostname(config) => write!(f, "{}", config),
            NodeTarget::Service(config) => write!(f, "{}", config),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IpAddressTargetConfig {
    pub ip: IpAddr,
    pub port: Option<u16> 
}

impl Default for IpAddressTargetConfig {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: Some(80)
        }
    }
}

impl std::fmt::Display for IpAddressTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port.unwrap_or(80))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HostnameTargetConfig {
    pub hostname: String, 
    pub port: Option<u16> 
}

impl Default for HostnameTargetConfig {
    fn default() -> Self {
        Self {
            hostname: "example.com".to_string(),
            port: Some(80)
        }
    }
}

impl std::fmt::Display for HostnameTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hostname, self.port.unwrap_or(80))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ServiceTargetConfig {
    pub protocol: ApplicationProtocol,
    pub hostname: String,
    pub port: Option<u16>,
    pub path: Option<String>,
}

impl Default for ServiceTargetConfig {
    fn default() -> Self {
        Self {
            protocol: ApplicationProtocol::Http,
            hostname: "example.com".to_string(),
            port: Some(80),
            path: Some("/".to_string())
        }
    }
}


impl std::fmt::Display for ServiceTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.as_deref().unwrap_or("");
        let port = self.port.unwrap_or_else(|| self.protocol.default_port());
        
        // Only show port if it's not the default for this protocol
        if port == self.protocol.default_port() {
            write!(f, "{:?}://{}{}", self.protocol, self.hostname, path)
        } else {
            write!(f, "{:?}://{}:{}{}", self.protocol, self.hostname, port, path)
        }
    }
}

impl NodeTarget {
    pub fn variant_name(&self) -> String {
        NodeTargetDiscriminants::from(self).to_string()
    }

    pub fn as_ip_config(&self) -> Option<&IpAddressTargetConfig> {
        match self {
            NodeTarget::IpAddress(config) => Some(config),
            _ => None,
        }
    }
    
    pub fn as_hostname_config(&self) -> Option<&HostnameTargetConfig> {
        match self {
            NodeTarget::Hostname(config) => Some(config),
            _ => None,
        }
    }
    
    pub fn as_service_config(&self) -> Option<&ServiceTargetConfig> {
        match self {
            NodeTarget::Service(config) => Some(config),
            _ => None,
        }
    }
}

impl TypeMetadataProvider for NodeTarget {
    fn id(&self) -> String { 
        self.variant_name().to_string()
    }
    
    fn display_name(&self) -> &str {
        match self {
            NodeTarget::IpAddress(..) => "IP Address",
            NodeTarget::Hostname(..) => "Hostname",
            NodeTarget::Service(..) => "Service",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeTarget::IpAddress(..) => "Direct connection using IP address",
            NodeTarget::Hostname(..) => "Connect using domain name or hostname",
            NodeTarget::Service(..) => "Full service endpoint with protocol and path",
        }
    }
    
    fn category(&self) -> &str {
        ""
    }
    
    fn icon(&self) -> &str {
        ""
    }
    
    fn color(&self) -> &str {
        ""
    }
    
    fn metadata(&self) -> serde_json::Value {
        match self {
            NodeTarget::IpAddress(..) => serde_json::json!({
                "defaultConfig": {
                    "ip": "127.0.0.1",
                    "port": 80
                }
            }),
            NodeTarget::Hostname(..) => serde_json::json!({"defaultConfig": {
                "hostname": "example.com",
                "port": 80
            }}),
            NodeTarget::Service(..) => serde_json::json!({"defaultConfig": {
                "protocol": ApplicationProtocol::Http,
                "hostname": "127.0.0.1",
                "port": 80,
                "path": '/'
            }}),
        }
    }
}