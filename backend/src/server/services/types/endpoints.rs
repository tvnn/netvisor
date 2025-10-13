use crate::server::hosts::types::ports::PortBase;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{fmt::Display, net::IpAddr};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    EnumDiscriminants,
    EnumIter,
)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum ApplicationProtocol {
    #[default]
    Http,
    Https,
}

#[derive(Debug, Clone, Eq)]
pub struct Endpoint {
    pub protocol: ApplicationProtocol,
    pub ip: Option<IpAddr>,
    pub port_base: PortBase,
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointResponse {
    pub endpoint: Endpoint,
    pub response: String,
}

impl Endpoint {
    pub fn is_resolved(&self) -> bool {
        self.ip.is_some()
    }

    pub fn use_ip(&self, ip: IpAddr) -> Self {
        Self {
            protocol: self.protocol,
            ip: Some(ip),
            port_base: self.port_base,
            path: self.path.clone(),
        }
    }

    pub fn http(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint {
            protocol: ApplicationProtocol::Http,
            port_base: PortBase::Http,
            ip,
            path: Some(path.to_string()),
        }
    }

    pub fn https(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint {
            protocol: ApplicationProtocol::Https,
            port_base: PortBase::Https,
            ip,
            path: Some(path.to_string()),
        }
    }

    pub fn http_alt(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint {
            protocol: ApplicationProtocol::Http,
            port_base: PortBase::HttpAlt,
            ip,
            path: Some(path.to_string()),
        }
    }

    pub fn https_alt(ip: Option<IpAddr>, path: &str) -> Self {
        Endpoint {
            protocol: ApplicationProtocol::Https,
            port_base: PortBase::HttpsAlt,
            ip,
            path: Some(path.to_string()),
        }
    }

    pub fn from_refs(
        ip: Option<IpAddr>,
        protocol: &ApplicationProtocol,
        port_base: &PortBase,
        path: &Option<String>,
    ) -> Self {
        Endpoint {
            protocol: *protocol,
            ip,
            port_base: *port_base,
            path: path.clone(),
        }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ip {
            Some(ip) => {
                write!(
                    f,
                    "{}://{}:{}{}",
                    self.protocol.discriminant().to_string().to_lowercase(),
                    ip,
                    self.port_base.number(),
                    self.path.as_deref().unwrap_or("")
                )
            }
            None => Err(std::fmt::Error),
        }
    }
}

impl PartialEq for Endpoint {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol
            && self.ip == other.ip
            && self.port_base.number() == other.port_base.number()
            && self.path == other.path
    }
}

impl Hash for Endpoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.protocol.hash(state);
        self.ip.hash(state);
        self.port_base.hash(state);
        self.path.hash(state);
    }
}
