use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::{hosts::types::{targets::HostTarget}, interfaces::types::base::Interface, services::types::{ports::Port}};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct HostUpdateRequest {
    pub name: Option<String>,
    pub hostname: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub target: Option<HostTarget>,
    pub interfaces: Option<Vec<Interface>>,
    pub services: Option<Vec<Uuid>>,
    pub open_ports: Option<Vec<Port>>,
}