use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnmpServiceCapability {}
impl Default for SnmpServiceCapability { fn default() -> Self { Self {} } }