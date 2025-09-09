use serde::{Deserialize, Serialize};
use std::{fmt::{self, Formatter}, net::{IpAddr, Ipv4Addr}};
use strum_macros::{EnumIter, EnumDiscriminants, Display};

use crate::server::shared::types::{metadata::TypeMetadataProvider};

#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum HostTarget {
    IpAddress(IpAddressTargetConfig),
    Hostname(HostnameTargetConfig)
}

impl fmt::Display for HostTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HostTarget::Hostname(HostnameTargetConfig{hostname}) => write!(f,"{}", hostname),
            HostTarget::IpAddress(IpAddressTargetConfig{ip}) => write!(f,"{}", ip.to_string())
        }
    }
}

impl PartialEq for HostTarget {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HostTarget::Hostname(HostnameTargetConfig{hostname}), HostTarget::Hostname(HostnameTargetConfig{hostname: other_hostname})) => hostname == other_hostname,
            (HostTarget::IpAddress(IpAddressTargetConfig{ip}),  HostTarget::IpAddress(IpAddressTargetConfig{ip: other_ip})) => ip == other_ip,
            _ => false
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

impl HostTarget {
    pub fn variant_name(&self) -> String {
        HostTargetDiscriminants::from(self).to_string()
    }
}

impl TypeMetadataProvider for HostTarget {
    fn id(&self) -> String { 
        self.variant_name().to_string()
    }
    
    fn display_name(&self) -> &str {
        match self {
            HostTarget::IpAddress(..) => "IP Address",
            HostTarget::Hostname(..) => "Hostname",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            HostTarget::IpAddress(..) => "Connect using an IP address",
            HostTarget::Hostname(..) => "Connect using a hostname",
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
            HostTarget::IpAddress(..) => serde_json::json!({
                "defaultConfig": {
                    "ip": "127.0.0.1",
                }
            }),
            HostTarget::Hostname(..) => serde_json::json!({
                "defaultConfig": {
                    "hostname": "127.0.0.1",
                }
            }),
        }
    }
}