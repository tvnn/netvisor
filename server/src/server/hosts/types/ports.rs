use serde::{Deserialize, Serialize};
use validator::Validate;
use std::fmt::Display;
use std::hash::Hash;
use strum_macros::{Display, EnumDiscriminants, EnumIter, IntoStaticStr};
use uuid::Uuid;

use crate::server::shared::{
    constants::Entity,
    types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
};

#[derive(Debug, Clone, PartialOrd, Ord, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportProtocol {
    #[default]
    Udp,
    Tcp,
}

#[derive(Debug, Validate, Clone, Eq)]
pub struct Port {
    pub id: Uuid,
    pub base: PortBase,
}

impl Hash for Port {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base.hash(state);
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

#[derive(Debug, Clone, Eq, EnumDiscriminants, EnumIter, IntoStaticStr)]
#[strum_discriminants(derive(Display, Hash, EnumIter))]
pub enum PortBase {
    Ssh,
    Telnet,
    DnsUdp,
    DnsTcp,
    Samba,
    Nfs,
    Ftp,
    Ipp,
    LdpTcp,
    LdpUdp,
    Snmp,
    Rdp,
    Ntp,
    Rtsp,
    Dhcp,
    Http,
    HttpAlt,
    Https,
    HttpsAlt,
    Custom(PortConfig),
}

impl Hash for PortBase {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.config().hash(state);
    }
}

impl PartialEq for PortBase {
    fn eq(&self, other: &Self) -> bool {
        self.config() == other.config()
    }
}

#[derive(Debug, Clone, Validate, Default, Eq, Serialize, Deserialize)]
pub struct PortConfig {
    #[validate(range(min=1, max=65535))]
    pub number: u16,
    pub protocol: TransportProtocol,
}

impl PartialEq for PortConfig {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.protocol == other.protocol
    }
}

impl Hash for PortConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
        self.protocol.hash(state);
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let port_str = match self.base.protocol() {
            TransportProtocol::Tcp => "/tcp",
            TransportProtocol::Udp => "/udp",
        };
        write!(f, "{}{}", self.base.number(), port_str)
    }
}

impl Port {
    pub fn new(base: PortBase) -> Self {
        Self {
            id: Uuid::new_v4(),
            base,
        }
    }
}

impl PortBase {
    pub fn new(number: u16, protocol: TransportProtocol) -> Self {
        PortBase::Custom(PortConfig { number, protocol })
    }

    pub fn new_tcp(number: u16) -> Self {
        PortBase::Custom(PortConfig {
            number,
            protocol: TransportProtocol::Tcp,
        })
    }

    pub fn new_udp(number: u16) -> Self {
        PortBase::Custom(PortConfig {
            number,
            protocol: TransportProtocol::Udp,
        })
    }
    pub fn protocol(&self) -> TransportProtocol {
        self.config().protocol
    }

    pub fn number(&self) -> u16 {
        self.config().number
    }

    pub fn config(&self) -> PortConfig {
        match &self {
            PortBase::Ssh => PortConfig {
                number: 22,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Telnet => PortConfig {
                number: 23,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::DnsTcp => PortConfig {
                number: 53,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::DnsUdp => PortConfig {
                number: 53,
                protocol: TransportProtocol::Udp,
            },
            PortBase::Samba => PortConfig {
                number: 445,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Nfs => PortConfig {
                number: 2049,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Ftp => PortConfig {
                number: 21,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Ipp => PortConfig {
                number: 631,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::LdpTcp => PortConfig {
                number: 515,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::LdpUdp => PortConfig {
                number: 515,
                protocol: TransportProtocol::Udp,
            },
            PortBase::Snmp => PortConfig {
                number: 161,
                protocol: TransportProtocol::Udp,
            },
            PortBase::Rdp => PortConfig {
                number: 3389,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Ntp => PortConfig {
                number: 123,
                protocol: TransportProtocol::Udp,
            },
            PortBase::Rtsp => PortConfig {
                number: 554,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Dhcp => PortConfig {
                number: 67,
                protocol: TransportProtocol::Udp,
            },
            PortBase::Http => PortConfig {
                number: 80,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::HttpAlt => PortConfig {
                number: 8080,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Https => PortConfig {
                number: 443,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::HttpsAlt => PortConfig {
                number: 8443,
                protocol: TransportProtocol::Tcp,
            },
            PortBase::Custom(config) => config.clone(),
        }
    }
}

impl HasId for PortBase {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for PortBase {
    fn color(&self) -> &'static str {
        Entity::Port.color()
    }
    fn icon(&self) -> &'static str {
        Entity::Port.icon()
    }
}

impl TypeMetadataProvider for PortBase {
    fn name(&self) -> &'static str {
        self.id()
    }
    fn metadata(&self) -> serde_json::Value {
        let is_management = matches!(
            self,
            PortBase::Ssh
                | PortBase::Telnet
                | PortBase::Rdp
                | PortBase::Snmp
                | PortBase::Http
                | PortBase::Https
                | PortBase::HttpAlt
                | PortBase::HttpsAlt
        );

        let is_dns = matches!(self, PortBase::DnsUdp | PortBase::DnsTcp);

        let number = self.number();
        let protocol = self.protocol();

        let can_be_added = !matches!(self, PortBase::Custom(_));

        serde_json::json!({
            "is_management": is_management,
            "is_dns": is_dns,
            "can_be_added": can_be_added,
            "number": number,
            "protocol": protocol
        })
    }
}

impl Serialize for Port {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Port", 4)?;
        state.serialize_field("id", &self.id)?;

        // Flatten the base fields directly into the Port
        let config = self.base.config();
        state.serialize_field("number", &config.number)?;
        state.serialize_field("protocol", &config.protocol)?;
        state.serialize_field("type", &self.base.id())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Port {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use strum::IntoEnumIterator;

        #[derive(Deserialize)]
        struct TempPort {
            id: Uuid,
            number: u16,
            protocol: TransportProtocol,
            #[serde(rename = "type")]
            _port_type: String,
        }

        let temp = TempPort::deserialize(deserializer)?;

        // Try to find a matching predefined port
        let base = PortBase::iter()
            .find(|variant| {
                // Skip Custom variants during iteration
                if matches!(variant, PortBase::Custom(_)) {
                    return false;
                }
                let config = variant.config();
                config.number == temp.number && config.protocol == temp.protocol
            })
            .unwrap_or({
                // If no predefined port matches, create a Custom variant
                PortBase::Custom(PortConfig {
                    number: temp.number,
                    protocol: temp.protocol,
                })
            });

        Ok(Port { id: temp.id, base })
    }
}
