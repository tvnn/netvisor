use serde::{Deserialize, Serialize};
use std::{fmt::{self, Formatter}, net::{IpAddr, Ipv4Addr}};
use strum_macros::{EnumIter, EnumDiscriminants, Display};

use crate::server::shared::types::{metadata::TypeMetadataProvider};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum NodeTarget {
    IpAddress(IpAddressTargetConfig),
    Hostname(HostnameTargetConfig)
}

impl fmt::Display for NodeTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NodeTarget::Hostname(HostnameTargetConfig{hostname}) => write!(f,"{}", hostname),
            NodeTarget::IpAddress(IpAddressTargetConfig{ip}) => write!(f,"{}", ip.to_string())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IpAddressTargetConfig {
    pub ip: IpAddr
}

impl Default for IpAddressTargetConfig {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HostnameTargetConfig {
    pub hostname: String
}

impl Default for HostnameTargetConfig {
    fn default() -> Self {
        Self {
            hostname: "example.com".to_string(),
        }
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
            NodeTarget::Hostname(..) => "Hostname",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeTarget::IpAddress(..) => "Connect using an IP address",
            NodeTarget::Hostname(..) => "Connect using a hostname",
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
            NodeTarget::Hostname(..) => serde_json::json!({
                "defaultConfig": {
                    "hostname": "127.0.0.1",
                }
            }),
        }
    }
}