use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum HostTarget {
    ServiceBinding(ServiceBinding),
    Hostname,
    None,
}

#[derive(Debug, Clone, Serialize, Eq, Deserialize)]
pub struct ServiceBinding {
    pub port_id: Uuid,
    pub interface_id: Uuid,
    pub service_id: Uuid,
}

impl PartialEq for ServiceBinding {
    fn eq(&self, other: &Self) -> bool {
        self.interface_id == other.interface_id && self.port_id == other.port_id && self.service_id == other.service_id
    }
}

impl Hash for ServiceBinding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.port_id.hash(state);
        self.interface_id.hash(state);
        self.service_id.hash(state);
    }
}
