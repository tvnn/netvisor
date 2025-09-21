use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::server::{hosts::types::targets::HostTarget, interfaces::types::base::Interface, services::types::{ports::Port}};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct HostBase {
    pub name: String,
    pub hostname: Option<String>,
    pub description: Option<String>,
    pub target: HostTarget,
    pub interfaces: Vec<Interface>,
    pub services: Vec<Uuid>,
    pub open_ports: Vec<Port>,
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

    pub fn get_interface(&self, interface_id: &Uuid) -> Option<&Interface> {
        self.base.interfaces.iter().find(|i| &i.id == interface_id)
    }

    pub fn add_service(&mut self, service_id: Uuid) {        
        self.base.services.push(service_id);
    }
}