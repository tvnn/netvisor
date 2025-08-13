use serde::{Deserialize, Serialize};
use crate::core::test_types::BaseTestConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub target: String,
    pub port: Option<u16>,
    pub protocol: Option<String>, // 'http' | 'https'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectIpConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub target: String,  // IP address
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub target: String,
    pub port: Option<u16>,
    pub attempts: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellknownIpConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    // No additional fields - tests well-known IPs like 8.8.8.8, 1.1.1.1
}

impl Default for ConnectivityConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            target: String::new(),
            port: None,
            protocol: Some("http".to_string()),
        }
    }
}

impl Default for DirectIpConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            target: String::new(),
            port: 80,
        }
    }
}

impl Default for PingConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            target: String::new(),
            port: None,
            attempts: Some(4),
        }
    }
}

impl Default for WellknownIpConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
        }
    }
}