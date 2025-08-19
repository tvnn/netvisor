use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use crate::shared::types::ApplicationProtocol;
use strum_macros::{EnumIter, EnumDiscriminants, Display};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum NodeTarget {
    IpAddress(IpAddressTargetConfig),
    Hostname(HostnameTargetConfig),
    Service(ServiceTargetConfig)
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IpAddressTargetConfig {
    pub ip: IpAddr,
    pub port: Option<u16> 
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

impl std::fmt::Display for ServiceTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.as_deref().unwrap_or("");
        let port = self.port.unwrap_or_else(|| self.protocol.default_port());
        
        // Only show port if it's not the default for this protocol
        if port == self.protocol.default_port() {
            write!(f, "{}://{}{}", self.protocol, self.hostname, path)
        } else {
            write!(f, "{}://{}:{}{}", self.protocol, self.hostname, port, path)
        }
    }
}

impl NodeTarget {
    pub fn variant_name(&self) -> String {
        NodeTargetDiscriminants::from(self).to_string()
    }

    pub fn ip_template() -> Self {
        NodeTarget::IpAddress(IpAddressTargetConfig{
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST), port: Some(80)
        })
    }
    
    pub fn service_template() -> Self {
        NodeTarget::Service(ServiceTargetConfig {
            protocol: ApplicationProtocol::Http,
            hostname: String::new(),
            port: Some(80),
            path: None,
        })
    }

    pub fn hostname_template() -> Self {
        NodeTarget::Hostname(HostnameTargetConfig{
            hostname: String::new(),
            port: Some(80)
        })
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

    // pub fn get_target(&self) -> String {
    //     match &self {
    //         NodeTarget::IpAddress(IpAddressTargetConfig { ip, port }) => {
    //             format!("{}:{}", ip, port.unwrap_or(80))
    //         },
    //         NodeTarget::Hostname(HostnameTargetConfig { hostname, port }) => {
    //             format!("{}:{}", hostname, port.unwrap_or(80))
    //         },
    //         NodeTarget::Service(ServiceTargetConfig { protocol: _, hostname, port, path: _ }) => {
    //             format!("{}:{}", hostname, port.unwrap_or(80))
    //         },
    //     }
    // }
}