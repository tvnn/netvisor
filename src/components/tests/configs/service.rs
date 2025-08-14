use serde::{Deserialize, Serialize};
use crate::components::tests::types::BaseTestConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub target: String,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub expected_status: Option<u16>,
}

impl Default for ServiceHealthConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            target: String::new(),
            port: Some(80),
            path: Some("/".to_string()),
            expected_status: Some(200),
        }
    }
}