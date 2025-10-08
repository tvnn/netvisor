use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum HostTarget {
    ServiceBinding(ServiceBinding),
    Hostname,
    None,
}

#[derive(Debug, Clone, Serialize, Eq, Deserialize)]
pub struct ServiceBinding {
    pub binding_id: Uuid,
    pub service_id: Uuid,
}

impl PartialEq for ServiceBinding {
    fn eq(&self, other: &Self) -> bool {
        self.binding_id == other.binding_id && self.service_id == other.service_id
    }
}

impl Hash for ServiceBinding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.binding_id.hash(state);
        self.service_id.hash(state);
    }
}
