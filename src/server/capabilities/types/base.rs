use anyhow::Error;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use strum::{IntoDiscriminant, IntoEnumIterator};
use uuid::Uuid;
use crate::server::{capabilities::{types::forms::{CapabilityConfigForm}}, nodes::types::{targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}}, shared::{forms::{field_factory::FieldFactory, types::fields::ConfigField}, types::metadata::TypeMetadataProvider}};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum Capability {
    Http{name: String, port: Option<u16>, path: Option<String>},
    Https{name: String, port: Option<u16>, path: Option<String>},
    Ssh{name: String, port: Option<u16>},
    Dns{name: String, port: Option<u16>},
    Dhcp{name: String, port: Option<u16>},

    Wireguard{name: String, port: Option<u16>},
    Daemon{name: String, port: Option<u16>, daemon_id: Uuid},
}

impl CapabilityDiscriminants {
    fn is_system_assigned(&self) -> bool {
        match self {
            CapabilityDiscriminants::Daemon => true,
            _ => false
        }
    }

    pub fn discovery_ports(&self) -> Vec<u16> {
        match self {
            CapabilityDiscriminants::Http => vec!(80, 8080),
            CapabilityDiscriminants::Https => vec!(443, 8443),
            CapabilityDiscriminants::Ssh => vec!(22),
            CapabilityDiscriminants::Dns => vec!(53),
            CapabilityDiscriminants::Dhcp => vec!(67),
            CapabilityDiscriminants::Wireguard => vec!(51820),
            CapabilityDiscriminants::Daemon => vec!(3001),
        }
    }
}

impl Capability {
    pub fn generate_form(&self) -> CapabilityConfigForm {
        CapabilityConfigForm {
            capability_info: self.discriminant().to_metadata(),
            capability_fields:self.generate_capability_fields(),
            warnings: vec![],
            errors: vec![],
            system_assigned: self.discriminant().is_system_assigned()
        }
    }

    fn generate_capability_fields(&self) -> Vec<ConfigField> {
        match self {
            Capability::Http{port, ..} => { vec![FieldFactory::port(*port), FieldFactory::path()] },
            Capability::Daemon{port, ..} => { vec![FieldFactory::port(*port)] },
            Capability::Https{port, ..} => { vec![ FieldFactory::port(*port), FieldFactory::path() ] },
            Capability::Dhcp{port, ..} => { vec![FieldFactory::port(*port)] },
            Capability::Ssh{port, ..} => { vec![FieldFactory::port(*port)] },
            Capability::Wireguard{port, ..} => { vec![FieldFactory::port(*port)] },
            Capability::Dns{port, ..} => { vec![FieldFactory::port(*port)] }
        }
    }

    pub fn from_port(port: Option<u16>) -> Option<Self> {
        CapabilityDiscriminants::iter()
            .find_map(|discriminant| {
                if port.is_some() && discriminant.discovery_ports().contains(&port.unwrap()) { 
                    return Some(
                        match discriminant {
                                CapabilityDiscriminants::Http => Self::Http{port, name: discriminant.display_name().to_string(), path:Some("/".to_string())},
                                CapabilityDiscriminants::Https => Self::Https{port, name: discriminant.display_name().to_string(), path:Some("/".to_string())},
                                CapabilityDiscriminants::Ssh => Self::Ssh{port, name: discriminant.display_name().to_string()},
                                CapabilityDiscriminants::Dns => Self::Dns{port, name: discriminant.display_name().to_string()},
                                CapabilityDiscriminants::Dhcp => Self::Dhcp{port, name: discriminant.display_name().to_string()},
                                CapabilityDiscriminants::Wireguard => Self::Wireguard{port, name: discriminant.display_name().to_string()},
                                CapabilityDiscriminants::Daemon => Self::Daemon{port, name: discriminant.display_name().to_string(), daemon_id: Uuid::nil()},
                            }
                    )
                }
                None
            })
    }

    pub fn discovery_ports() -> Vec<u16> {
        CapabilityDiscriminants::iter()
            .map(|discriminant| discriminant.discovery_ports())
            .flatten()
            .collect()
    }

    pub fn as_endpoint(&self, target: &NodeTarget) -> Option<String> {
        match self {
            Capability::Http{port, path, ..} => { Capability::get_endpoint(port, target, Some("http://".to_string()), path).ok() }
            Capability::Daemon{port, ..} => { Capability::get_endpoint(port, target, Some("http://".to_string()), &None).ok() }
            Capability::Https{port, path, ..} => { Capability::get_endpoint(port, target, Some("https://".to_string()), path).ok() }
            _ => None
        }
    }

    fn get_endpoint(port: &Option<u16>, target: &NodeTarget, protocol: Option<String>, path: &Option<String>) -> Result<String, Error> {
        if port.is_none() {return Err(Error::msg("Selected capability does not have a port"))}

        let target = match target {
            NodeTarget::Hostname(HostnameTargetConfig{hostname}) => hostname.to_string(),
            NodeTarget::IpAddress(IpAddressTargetConfig{ip}) => ip.to_string()
        };

        Ok(format!("{}{}:{}{}", 
            protocol.unwrap_or("http://".to_string()), 
            target, 
            port.unwrap_or(80),
            path.clone().unwrap_or("".to_string())
        ))
    }

}

impl TypeMetadataProvider for CapabilityDiscriminants {
    fn id(&self) -> String { 
        self.to_string()
    } 

    fn display_name(&self) -> &str {
        match self {
            CapabilityDiscriminants::Ssh => "SSH",
            CapabilityDiscriminants::Http => "HTTP",
            CapabilityDiscriminants::Https => "HTTPS",
            CapabilityDiscriminants::Wireguard => "Wireguard VPN",
            CapabilityDiscriminants::Daemon => "NetVisor Daemon",
            CapabilityDiscriminants::Dns => "DNS",
            CapabilityDiscriminants::Dhcp => "DHCP",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            CapabilityDiscriminants::Ssh  => "Remote command-line access for management and troubleshooting",
            CapabilityDiscriminants::Http  => "Web service providing HTTP content",
            CapabilityDiscriminants::Https  => "Secure web service providing HTTPS content", 
            CapabilityDiscriminants::Dns  => "Domain name resolution service",
            CapabilityDiscriminants::Wireguard  => "Modern VPN service using WireGuard protocol",
            CapabilityDiscriminants::Daemon  => "NetVisor daemon for enhanced network diagnostics",
            CapabilityDiscriminants::Dhcp => "Dynamic host configuration protocol service",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            CapabilityDiscriminants::Ssh  => "Remote Access",
            CapabilityDiscriminants::Http | CapabilityDiscriminants::Https => "Web Services",
            CapabilityDiscriminants::Wireguard => "Security",
            CapabilityDiscriminants::Dns | CapabilityDiscriminants::Dhcp => "Network Infrastructure",
            CapabilityDiscriminants::Daemon => "NetVisor",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            CapabilityDiscriminants::Ssh   => "Terminal",
            CapabilityDiscriminants::Http | CapabilityDiscriminants::Https => "Globe",
            CapabilityDiscriminants::Wireguard => "Lock",
            CapabilityDiscriminants::Dns => "Search",
            CapabilityDiscriminants::Daemon => "RectangleGoggles",
            CapabilityDiscriminants::Dhcp => "Router"
        }
    }
    
    fn color(&self) -> &str {
        match self {
            CapabilityDiscriminants::Ssh  => "green",
            CapabilityDiscriminants::Http | CapabilityDiscriminants::Https => "blue",
            CapabilityDiscriminants::Wireguard => "orange",
            CapabilityDiscriminants::Dns | CapabilityDiscriminants::Dhcp => "yellow",
            CapabilityDiscriminants::Daemon  => "purple",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({"system_assigned":self.is_system_assigned()})
    }
}