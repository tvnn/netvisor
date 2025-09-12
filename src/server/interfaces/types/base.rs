use std::{net::{IpAddr}};
use mac_address::{MacAddress};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct InterfaceBase {
    pub subnet_id: Uuid,
    pub ip_address: IpAddr,
    pub mac_address: Option<MacAddress>,
    pub name: String,
    pub is_primary: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Interface {
    pub id: Uuid,
    #[serde(flatten)]
    pub base: InterfaceBase,
}

impl Interface {
    pub fn new(base: InterfaceBase) -> Self{
        Self {
            id: Uuid::new_v4(),
            base
        }
    }
}


// impl Interface {
//     pub fn from_host(host: Host, ) {
//         Self {

//         }
//     }
// }