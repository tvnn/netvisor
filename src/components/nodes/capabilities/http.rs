use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HttpServiceCapability {}
impl Default for HttpServiceCapability { fn default() -> Self { Self {} } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HttpsServiceCapability {}
impl Default for HttpsServiceCapability { fn default() -> Self { Self {} } }