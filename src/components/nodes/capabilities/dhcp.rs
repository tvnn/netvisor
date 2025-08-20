use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DhcpServiceCapability {}
impl Default for DhcpServiceCapability { fn default() -> Self { Self {} } }