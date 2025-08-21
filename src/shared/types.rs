use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, EnumDiscriminants, Hash)]
#[strum_discriminants(derive(Display))]
pub enum TransportProtocol {
    Tcp,
    Udp
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, EnumDiscriminants, Hash)]
#[strum_discriminants(derive(Display))]
pub enum ApplicationProtocol {
    Http,
    Https,
    Ftp
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