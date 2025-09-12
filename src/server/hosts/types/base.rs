use std::{net::{IpAddr}};

use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::server::{hosts::types::targets::HostTarget, interfaces::types::base::Interface, services::types::{base::Service, ports::Port, types::ServiceType}};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct HostBase {
    pub name: String,
    pub hostname: Option<String>,
    pub description: Option<String>,
    pub target: HostTarget,
    pub interfaces: Vec<Interface>,
    pub services: Vec<Service>,
    pub open_ports: Vec<Port>,
    pub groups: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Host {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: HostBase,
}

impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        
        let macs_a: Vec<Option<MacAddress>> = self.base.interfaces.iter().map(|s| s.base.mac_address).collect();
        let macs_b: Vec<Option<MacAddress>> = other.base.interfaces.iter().map(|s| s.base.mac_address).collect();

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

        let subnet_ip_match = self.base.interfaces.iter().any(|subnet_a| {
            other.base.interfaces.iter().any(|subnet_b| {
                subnet_a.base.subnet_id == subnet_b.base.subnet_id && subnet_a.base.ip_address == subnet_b.base.ip_address
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
            base,
        }
    }

    pub fn default_service(&self) -> Option<&Service> {
        self.base.services.first()
    }

    pub fn primary_interface(&self) -> Option<&Interface> {
        self.base.interfaces.iter().find_map(|s| if s.base.is_primary {Some(s)} else {None}).or(self.base.interfaces.first())
    }

    pub fn default_ip(&self) -> Option<IpAddr> {
        match self.primary_interface() {
            Some(subnet) => Some(subnet.base.ip_address),
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

    pub fn has_service(&self, service_type: ServiceType) -> bool{
        self.base.services.iter().any(|s| s.base.service_type == service_type)
    }

    pub fn get_service(&self, service_type: ServiceType) -> Option<&Service>{
        self.base.services.iter().find(|s| s.base.service_type == service_type)
    }

    pub fn add_service(&mut self, service: Service) {        
        self.base.services.push(service);
    }
}