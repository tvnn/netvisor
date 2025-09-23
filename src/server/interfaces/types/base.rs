use std::{net::{IpAddr}};
use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct InterfaceBase {
    pub subnet_id: Uuid,
    pub ip_address: IpAddr,
    pub mac_address: Option<MacAddress>,
    pub name: Option<String>,
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