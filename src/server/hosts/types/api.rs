use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::hosts::types::base::HostSubnetMembership;
use crate::server::{hosts::types::{base::Host, targets::HostTarget}, services::types::{base::Service, ports::Port}, subnets::types::base::{Subnet}};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct HostUpdateRequest {
    pub name: Option<String>,
    pub hostname: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub target: Option<HostTarget>,
    pub subnets: Option<Vec<HostSubnetMembership>>,
    pub services: Option<Vec<Service>>,
    pub open_ports: Option<Vec<Port>>,
    pub groups: Option<Vec<Uuid>>,
}

impl HostUpdateRequest {
    pub fn from_group_change(groups: Vec<Uuid>) -> Self {
        Self {
            name: None,
            hostname: None,
            description: None,
            target: None,
            subnets: None,
            services: None,
            open_ports: None,
            groups: Some(groups),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct UpdateHostResponse {
    pub host: Host,
    pub subnet_changes: HostSubnetRelationshipChange
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct HostSubnetRelationshipChange {
    pub new_gateway: Vec<Subnet>,
    pub no_longer_gateway: Vec<Subnet>,
    pub new_dns_resolver: Vec<Subnet>,
    pub no_longer_dns_resolver: Vec<Subnet>
}