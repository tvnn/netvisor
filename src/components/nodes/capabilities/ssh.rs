use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SshAccessCapability {}
impl Default for SshAccessCapability { fn default() -> Self { Self {} } }