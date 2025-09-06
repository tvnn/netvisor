use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};

#[derive(Debug, Clone, Default, Display, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum ApplicationProtocol {
    #[default]
    Http,
    Https
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Port {
    pub number: u16,
    pub udp: bool,
    pub tcp: bool
}

impl Port {
    
    pub const SSH: Port = Port{number: 22, tcp: true, udp: false};
    pub const DNS: Port = Port{number: 53, tcp: true, udp: true};
    pub const SAMBA: Port = Port{number: 445, tcp: true, udp: false};
    pub const NFS: Port = Port{number: 2049, tcp: true, udp: false};
    pub const FTP: Port = Port{number: 21, tcp: true, udp: false};
    pub const IPP: Port = Port{number: 631, tcp: true, udp: false};
    pub const SNMP: Port = Port{number: 161, tcp: true, udp: false};
    pub const RDP: Port = Port{number: 3389, tcp: true, udp: false};
    pub const RTSP: Port = Port{number: 554, tcp: true, udp: false};
    pub const DHCP: Port = Port{number: 67, tcp: false, udp: true};
    pub const HTTP: Port = Port{number: 80, tcp: true, udp: false};
    pub const HTTPALT: Port = Port{number: 8080, tcp: true, udp: false};
    pub const HTTPS: Port = Port{number: 443, tcp: true, udp: false};
    pub const HTTPSALT: Port = Port{number: 8443, tcp: true, udp: false};

    pub fn new(number: u16, udp: bool, tcp: bool) -> Self {
        Self {
            number,
            udp,
            tcp
        }
    }

    pub fn new_tcp(number: u16) -> Self {
        Self {
            number,
            tcp:true,
            udp:false
        }
    }

    pub fn new_udp(number: u16) -> Self {
        Self {
            number,
            tcp:false,
            udp:true
        }
    }
}