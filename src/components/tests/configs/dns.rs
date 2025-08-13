use serde::{Deserialize, Serialize};
use crate::core::test_types::BaseTestConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResolutionConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsOverHttpsConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub target: String,     // DoH endpoint URL
    pub domain: String,     // Domain to resolve
    pub service_type: Option<String>, // 'cloudflare' | 'google' | 'custom'
}

impl Default for DnsResolutionConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            domain: "example.com".to_string(),
        }
    }
}

impl Default for DnsOverHttpsConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            target: "https://1.1.1.1/dns-query".to_string(),
            domain: "example.com".to_string(),
            service_type: Some("cloudflare".to_string()),
        }
    }
}