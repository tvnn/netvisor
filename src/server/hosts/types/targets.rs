use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag="type", content="config")]
pub enum HostTarget {
    Interface(Uuid),
    ExternalIp(IpAddr),
    Hostname
}

// #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub struct IpAddressTargetConfig {
//     pub ip: IpAddr
// }

// impl Default for IpAddressTargetConfig {
//     fn default() -> Self {
//         Self {
//             ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
//         }
//     }
// }

// #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub struct HostnameTargetConfig {
//     pub hostname: String
// }

// impl Default for HostnameTargetConfig {
//     fn default() -> Self {
//         Self {
//             hostname: "example.com".to_string(),
//         }
//     }
// }

// impl HostTarget {
//     pub fn variant_name(&self) -> String {
//         HostTargetDiscriminants::from(self).to_string()
//     }
// }