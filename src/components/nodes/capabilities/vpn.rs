use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VpnServiceCapability {}
impl Default for VpnServiceCapability { fn default() -> Self { Self {} } }