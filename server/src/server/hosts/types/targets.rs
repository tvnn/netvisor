use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag="type", content="config")]
pub enum HostTarget {
    ServiceBinding(ServiceBinding),
    Hostname,
    None
}

#[derive(Debug, Clone, Serialize, Hash, Eq, Deserialize)]
pub struct ServiceBinding {
    pub port_id: Uuid,
    pub interface_id: Uuid,
    pub service_id: Uuid
}

impl PartialEq for ServiceBinding {
    fn eq(&self, other: &Self) -> bool {
        self.interface_id == other.interface_id && self.port_id == other.port_id
    }
}
