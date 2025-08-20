use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SmtpServiceCapability {}
impl Default for SmtpServiceCapability { fn default() -> Self { Self {} } }