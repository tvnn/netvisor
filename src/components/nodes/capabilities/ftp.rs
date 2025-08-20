use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FtpServiceCapability {}
impl Default for FtpServiceCapability { fn default() -> Self { Self {} } }