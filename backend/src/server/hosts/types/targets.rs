use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::server::services::types::bindings::ServiceBinding;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum HostTarget {
    ServiceBinding(ServiceBinding),
    Hostname,
    None,
}
