use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum TransportProtocol {
    Tcp,
    Udp
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ApplicationProtocol {
    Http,
    Https,
    Ftp
}

impl std::fmt::Display for ApplicationProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let protocol_str = match self {
            ApplicationProtocol::Http => "http",
            ApplicationProtocol::Https => "https", 
            ApplicationProtocol::Ftp => "ftp",
        };
        write!(f, "{}", protocol_str)
    }
}

impl ApplicationProtocol {
    pub fn default_port(&self) -> u16 {
        match self {
            ApplicationProtocol::Http => 80,
            ApplicationProtocol::Https => 443,
            ApplicationProtocol::Ftp => 21,
        }
    }
}