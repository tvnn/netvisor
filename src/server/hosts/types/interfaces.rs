use std::net::{IpAddr, Ipv4Addr};
use mac_address::{MacAddress};
use rand::{Rng};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

use crate::server::subnets::types::base::Subnet;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct InterfaceBase {
    pub subnet_id: Uuid,
    pub ip_address: IpAddr,
    pub mac_address: Option<MacAddress>,
    pub name: Option<String>,
}

impl InterfaceBase {
    pub fn new_internet(internet_subnet: &Subnet) -> Self {

        let ip_address = IpAddr::V4(Ipv4Addr::new(203, 0, 113, rand::rng().random_range(1..255)));

        Self {
            subnet_id: internet_subnet.id,
            ip_address,
            mac_address: None,
            name: Some("Internet".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Interface {
    pub id: Uuid,
    #[serde(flatten)]
    pub base: InterfaceBase,
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.base.ip_address == other.base.ip_address && 
                self.base.subnet_id == other.base.subnet_id
    }
}

impl Interface {
    pub fn new(base: InterfaceBase) -> Self{
        Self {
            id: Uuid::new_v4(),
            base
        }
    }
}