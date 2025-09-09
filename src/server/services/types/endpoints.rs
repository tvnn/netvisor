use std::{fmt::{Display}, net::IpAddr};
use serde::{Deserialize, Serialize};
use strum::{IntoDiscriminant};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use crate::server::services::types::ports::{Port};

#[derive(Debug, Clone, Default, Display, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum ApplicationProtocol {
    #[default]
    Http,
    Https
}

#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
    pub protocol: ApplicationProtocol,
    pub ip: Option<IpAddr>,
    pub port: Port,
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointResponse {
    pub endpoint: Endpoint,
    pub response: String
}

impl Endpoint {

    pub fn is_resolved(&self) -> bool {
        self.ip.is_some()
    }

    pub fn use_ip(&self, ip: IpAddr) -> Self {
        Self {
            protocol: self.protocol.clone(),
            ip: Some(ip),
            port: self.port.clone(),
            path: self.path.clone()
        }
    }

    pub fn http(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint { protocol: ApplicationProtocol::Http, port: Port::HTTP, ip: ip.clone(), path: Some(path.to_string())}
    }

    pub fn https(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint { protocol: ApplicationProtocol::Https, port: Port::HTTPS, ip: ip.clone(), path: Some(path.to_string())}
    }

    pub fn http_alt(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint { protocol: ApplicationProtocol::Http, port: Port::HTTPALT, ip: ip.clone(), path: Some(path.to_string())}
    }

    pub fn https_alt(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint { protocol: ApplicationProtocol::Https, port: Port::HTTPSALT, ip: ip.clone(), path: Some(path.to_string())}
    }

    pub fn from_refs(ip: Option<IpAddr>, protocol: &ApplicationProtocol, port: &Port, path: &Option<String>) -> Self {
        Endpoint { protocol: protocol.clone(), ip: ip.clone(), port: port.clone(), path: path.clone()}
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ip {
            Some(ip) => {
                write!(f, "{}://{}:{}{}", 
                    self.protocol.discriminant().to_string().to_lowercase(), 
                    ip,
                    self.port.number,
                    self.path.as_deref().unwrap_or("")
                )
            }
            None => Err(std::fmt::Error)
        }
    }
}

impl PartialEq for Endpoint {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol && 
        self.ip == other.ip && 
        self.port.number == other.port.number && 
        self.path == other.path
    }
}
