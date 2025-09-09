use std::{net::{IpAddr}};

use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::IntoDiscriminant;
use crate::server::{hosts::types::targets::HostTarget, services::types::{base::{Service, ServiceDiscriminants}, ports::Port}};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct HostBase {
    pub name: String,
    pub hostname: Option<String>,
    pub description: Option<String>,
    pub target: HostTarget,
    pub subnets: Vec<HostSubnetMembership>,
    pub services: Vec<Service>,
    pub open_ports: Vec<Port>,
    pub groups: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct HostSubnetMembership {
    pub subnet_id: Uuid,
    pub ip_address: IpAddr,
    pub mac_address: Option<MacAddress>,
    pub default: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Host {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub base: HostBase,
}

impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        
        let macs_a: Vec<Option<MacAddress>> = self.base.subnets.iter().map(|s| s.mac_address).collect();
        let macs_b: Vec<Option<MacAddress>> = other.base.subnets.iter().map(|s| s.mac_address).collect();

        let mac_match = macs_a.iter().any(|mac_a| {
            macs_b.iter().any(|mac_b| {
                match (mac_a, mac_b) {
                    (Some(a), Some(b)) => !vec!(
                        MacAddress::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                        MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
                    ).contains(&a) && a == b,
                    (_, _) => false
                }
            })
        });

        let subnet_ip_match = self.base.subnets.iter().any(|subnet_a| {
            other.base.subnets.iter().any(|subnet_b| {
                subnet_a.subnet_id == subnet_b.subnet_id && subnet_a.ip_address == subnet_b.ip_address
            })
        });

        mac_match || subnet_ip_match
    }
}

impl Host {
    pub fn new(base: HostBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            last_seen: None,
            base,
        }
    }

    pub fn default_subnet(&self) -> Option<&HostSubnetMembership> {
        self.base.subnets.iter().find_map(|s| if s.default {Some(s)} else {None})
    }

    pub fn default_ip(&self) -> Option<IpAddr> {
        match self.default_subnet() {
            Some(subnet) => Some(subnet.ip_address),
            None => None
        }
    }

    pub fn add_to_group(&mut self, group_id: Uuid) -> Self {
        if !self.base.groups.contains(&group_id) {
            self.base.groups.push(group_id);
            self.updated_at = chrono::Utc::now();
        }
        self.clone()
    }
    
    pub fn remove_from_group(&mut self, group_id: &Uuid) {
        self.base.groups.retain(|id| id != group_id);
        self.updated_at = chrono::Utc::now();
    }

    pub fn has_service(&self, service_discriminant: ServiceDiscriminants) -> bool{
        self.base.services.iter().any(|c| c.discriminant() == service_discriminant)
    }

    pub fn get_service(&self, service_discriminant: ServiceDiscriminants) -> Option<&Service>{
        self.base.services.iter().find(|c| c.discriminant() == service_discriminant)
    }

    pub fn add_service(&mut self, service: Service) {        
        self.base.services.push(service);
    }
}