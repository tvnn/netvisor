use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};

#[derive(Debug, Clone, Default, Display, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum TransportProtocol {
    #[default]
    Udp,
    Tcp
}

#[derive(Debug, Clone, Default, Eq, Hash, Serialize, Deserialize)]
pub struct Port {
    pub number: u16,
    pub protocol: TransportProtocol
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.protocol == other.protocol
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let port_str = match self.protocol {
            TransportProtocol::Tcp => "/tcp",
            TransportProtocol::Udp => "/udp",
        };
        write!(f, "{}{}", self.number, port_str)
    }
}

impl Port {
    
    pub const SSH: Port = Port{number: 22, protocol: TransportProtocol::Tcp};
    pub const TELNET: Port = Port{number: 23, protocol: TransportProtocol::Tcp};
    pub const DNS_UDP: Port = Port{number: 53, protocol: TransportProtocol::Tcp};
    pub const DNS_TCP: Port = Port{number: 53, protocol: TransportProtocol::Udp};
    pub const SAMBA: Port = Port{number: 445, protocol: TransportProtocol::Tcp};
    pub const NFS: Port = Port{number: 2049, protocol: TransportProtocol::Tcp};
    pub const FTP: Port = Port{number: 21, protocol: TransportProtocol::Tcp};
    pub const IPP: Port = Port{number: 631, protocol: TransportProtocol::Tcp};
    pub const LDP_TCP: Port = Port{number: 515, protocol: TransportProtocol::Tcp};
    pub const LDP_UDP: Port = Port{number: 515, protocol: TransportProtocol::Udp};
    pub const SNMP: Port = Port{number: 161, protocol: TransportProtocol::Udp};
    pub const RDP: Port = Port{number: 3389, protocol: TransportProtocol::Tcp};
    pub const NTP: Port = Port{number: 123, protocol: TransportProtocol::Udp};
    pub const RTSP: Port = Port{number: 554, protocol: TransportProtocol::Tcp};
    pub const DHCP: Port = Port{number: 67, protocol: TransportProtocol::Udp};
    pub const HTTP: Port = Port{number: 80, protocol: TransportProtocol::Tcp};
    pub const HTTPALT: Port = Port{number: 8080, protocol: TransportProtocol::Tcp};
    pub const HTTPS: Port = Port{number: 443, protocol: TransportProtocol::Tcp};
    pub const HTTPSALT: Port = Port{number: 8443, protocol: TransportProtocol::Tcp};

    pub fn new(number: u16, protocol: TransportProtocol) -> Self {
        Self {
            number,
            protocol
        }
    }

    pub fn new_tcp(number: u16) -> Self {
        Self {
            number,
            protocol: TransportProtocol::Tcp
        }
    }

    pub fn new_udp(number: u16) -> Self {
        Self {
            number,
            protocol: TransportProtocol::Udp
        }
    }
}

pub const MANAGEMENT_INTERFACE_PORTS: &[Port; 5] = &[Port::SSH, Port::HTTP, Port::HTTPALT, Port::HTTPS, Port::HTTPSALT];