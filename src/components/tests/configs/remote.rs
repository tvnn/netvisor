use serde::{Deserialize, Serialize};
use crate::core::test_types::BaseTestConfig;

// Future daemon-based execution (Phase 5)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonCommandConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub command: String,
    pub requires_confirmation: Option<bool>,
    pub rollback_command: Option<String>,
    pub expected_output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshScriptConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub command: String,
    pub ssh_target: String,  // user@host format
    pub requires_confirmation: Option<bool>,
    pub rollback_command: Option<String>,
    pub expected_output: Option<String>,
}

impl Default for DaemonCommandConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            command: String::new(),
            requires_confirmation: Some(true),
            rollback_command: None,
            expected_output: None,
        }
    }
}

impl Default for SshScriptConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            command: String::new(),
            ssh_target: String::new(),
            requires_confirmation: Some(true),
            rollback_command: None,
            expected_output: None,
        }
    }
}