use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};

use crate::{server::nodes::capabilities::{
    dns::*, http::*, ssh::SshAccessCapability, vpn::*
}, server::shared::types::metadata::TypeMetadataProvider, 
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Serialize, Deserialize))]
pub enum NodeCapability {
    SshAccess(SshAccessCapability),
    HttpService(HttpServiceCapability),
    HttpsService(HttpsServiceCapability),
    DnsService(DnsServiceCapability),
    VpnService(VpnServiceCapability),
}

impl NodeCapability {

    /// Get variant name as string
    pub fn variant_name(&self) -> String {
        NodeCapabilityDiscriminants::from(self).to_string()
    }

    // /// Helper methods to access specific capability implementations
    // pub fn as_dns_capability(&self) -> Option<&DnsServiceCapability> {
    //     match self {
    //         NodeCapability::DnsService(capability) => Some(capability),
    //         _ => None,
    //     }
    // }

    /// Get the default port associated with this capability (for auto-detection)
    pub fn default_port(&self) -> Option<u16> {
        match self {
            NodeCapability::SshAccess(_) => Some(22),
            NodeCapability::HttpService(_) => Some(80),
            NodeCapability::HttpsService(_) => Some(443),
            NodeCapability::VpnService(_) => Some(51820), // Wireguard default
            NodeCapability::DnsService(_) => Some(53),
        }
    }

    /// Create capability from discovered port (for auto-detection)
    pub fn from_port(port: u16) -> Option<Self> {
        match port {
            22 => Some(NodeCapability::SshAccess(SshAccessCapability {  })),
            80 => Some(NodeCapability::HttpService(HttpServiceCapability {  })),
            443 => Some(NodeCapability::HttpsService(HttpsServiceCapability {  })),
            53 => Some(NodeCapability::DnsService(DnsServiceCapability {  })),
            1194 | 1723 | 500 | 4500 | 51820 => Some(NodeCapability::VpnService(VpnServiceCapability {  })),
            _ => None,
        }
    }
}

impl TypeMetadataProvider for NodeCapability {
    fn id(&self) -> String { 
        self.discriminant().to_string()
    }
    
    fn display_name(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "SSH Access",
            NodeCapability::HttpService(_) => "HTTP Service",
            NodeCapability::HttpsService(_) => "HTTPS Service",
            NodeCapability::VpnService(_) => "VPN Service",
            NodeCapability::DnsService(_) => "DNS Service",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "Remote command-line access for management and troubleshooting",
            NodeCapability::HttpService(_) => "Web interface or API accessible over HTTP",
            NodeCapability::HttpsService(_) => "Secure web interface or API accessible over HTTPS",
            NodeCapability::VpnService(_) => "VPN server for secure remote access",
            NodeCapability::DnsService(_) => "Domain name resolution service",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "Remote Access",
            NodeCapability::HttpService(_) | NodeCapability::HttpsService(_) => "Web Services",
            NodeCapability::VpnService(_) => "Security",
            NodeCapability::DnsService(_) => "Network Infrastructure",

        }
    }
    
    fn icon(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "Terminal",
            NodeCapability::HttpService(_) | NodeCapability::HttpsService(_) => "Globe",
            NodeCapability::VpnService(_) => "Lock",
            NodeCapability::DnsService(_) => "Search",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeCapability::SshAccess(_) => "green",
            NodeCapability::HttpService(_) | NodeCapability::HttpsService(_) => "blue",
            NodeCapability::VpnService(_) => "orange",
            NodeCapability::DnsService(_) => "purple",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}

// Serialization: Vec<NodeCapability> -> Vec<NodeCapabilityDiscriminants>
pub fn serialize_capabilities_as_discriminants<S>(
    capabilities: &Vec<NodeCapability>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let discriminants: Vec<NodeCapabilityDiscriminants> = capabilities
        .iter()
        .map(|cap| cap.into())
        .collect();
    discriminants.serialize(serializer)
}

// Serialization for Option<Vec<NodeCapability>>: Option<Vec<NodeCapability>> -> Option<Vec<NodeCapabilityDiscriminants>>
pub fn serialize_optional_capabilities_as_discriminants<S>(
    capabilities: &Option<Vec<NodeCapability>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match capabilities {
        Some(caps) => {
            let discriminants: Vec<NodeCapabilityDiscriminants> = caps
                .iter()
                .map(|cap| cap.into())
                .collect();
            Some(discriminants).serialize(serializer)
        }
        None => None::<Vec<NodeCapabilityDiscriminants>>.serialize(serializer),
    }
}

// Deserialization: Vec<NodeCapabilityDiscriminants> -> Vec<NodeCapability>
pub fn deserialize_capabilities_from_discriminants<'de, D>(
    deserializer: D,
) -> Result<Vec<NodeCapability>, D::Error>
where
    D: Deserializer<'de>,
{
    let discriminants: Vec<NodeCapabilityDiscriminants> = Vec::deserialize(deserializer)?;
    let capabilities = discriminants
        .into_iter()
        .map(|d| match d {
            NodeCapabilityDiscriminants::SshAccess => NodeCapability::SshAccess(SshAccessCapability {  }),
            NodeCapabilityDiscriminants::HttpService => NodeCapability::HttpService(HttpServiceCapability {  }),
            NodeCapabilityDiscriminants::HttpsService => NodeCapability::HttpsService(HttpsServiceCapability {  }),
            NodeCapabilityDiscriminants::DnsService => NodeCapability::DnsService(DnsServiceCapability {  }),
            NodeCapabilityDiscriminants::VpnService => NodeCapability::VpnService(VpnServiceCapability {  }),
        })
        .collect();
    Ok(capabilities)
}

// Deserialization for Option<Vec<NodeCapability>>: Option<Vec<NodeCapabilityDiscriminants>> -> Option<Vec<NodeCapability>>
pub fn deserialize_optional_capabilities_from_discriminants<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<NodeCapability>>, D::Error>
where
    D: Deserializer<'de>,
{
    let option_discriminants: Option<Vec<NodeCapabilityDiscriminants>> = Option::deserialize(deserializer)?;
    
    match option_discriminants {
        Some(discriminants) => {
            let capabilities: Vec<NodeCapability> = discriminants
                .into_iter()
                .map(|d| match d {
                    NodeCapabilityDiscriminants::SshAccess => NodeCapability::SshAccess(SshAccessCapability {  }),
                    NodeCapabilityDiscriminants::HttpService => NodeCapability::HttpService(HttpServiceCapability {  }),
                    NodeCapabilityDiscriminants::HttpsService => NodeCapability::HttpsService(HttpsServiceCapability {  }),
                    NodeCapabilityDiscriminants::DnsService => NodeCapability::DnsService(DnsServiceCapability {  }),
                    NodeCapabilityDiscriminants::VpnService => NodeCapability::VpnService(VpnServiceCapability {  }),
                })
                .collect();
            Ok(Some(capabilities))
        }
        None => Ok(None),
    }
}