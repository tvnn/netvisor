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

impl ApplicationProtocol {
    pub fn display_name(&self) -> String {
        match &self {
            ApplicationProtocol::Ftp => "ftp://".to_string(),
            ApplicationProtocol::Http => "http://".to_string(),
            ApplicationProtocol::Https => "https://".to_string()
        }
    }
}