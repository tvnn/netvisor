use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use strum_macros::{EnumIter, EnumDiscriminants, Display};

use crate::server::shared::types::{metadata::TypeMetadataProvider, protocols::ApplicationProtocol};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum NodeTarget {
    IpAddress(IpAddressTargetConfig),
    Url(UrlTargetConfig)
}

impl std::fmt::Display for NodeTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeTarget::IpAddress(config) => write!(f, "{}", config),
            NodeTarget::Url(config) => write!(f, "{}", config),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IpAddressTargetConfig {
    pub ip: IpAddr,
}

impl Default for IpAddressTargetConfig {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
}

impl std::fmt::Display for IpAddressTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://{}", self.ip)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UrlTargetConfig {
    pub protocol: ApplicationProtocol,
    pub hostname: String,
    pub path: Option<String>,
}

impl Default for UrlTargetConfig {
    fn default() -> Self {
        Self {
            protocol: ApplicationProtocol::Http,
            hostname: "example.com".to_string(),
            path: Some("/".to_string())
        }
    }
}


impl std::fmt::Display for UrlTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.as_deref().unwrap_or("");
        write!(f, "{:?}://{}{}", self.protocol, self.hostname, path)
    }
}

impl NodeTarget {
    pub fn variant_name(&self) -> String {
        NodeTargetDiscriminants::from(self).to_string()
    }
}

impl TypeMetadataProvider for NodeTarget {
    fn id(&self) -> String { 
        self.variant_name().to_string()
    }
    
    fn display_name(&self) -> &str {
        match self {
            NodeTarget::IpAddress(..) => "IP Address",
            NodeTarget::Url(..) => "URL",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeTarget::IpAddress(..) => "Direct connection using IP address",
            NodeTarget::Url(..) => "Connect using a URL",
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
                }
            }),
            NodeTarget::Url(..) => serde_json::json!({
                "defaultConfig": {
                    "protocol": ApplicationProtocol::Http,
                    "hostname": "127.0.0.1",
                    "path": '/'
                }
            }),
        }
    }
}