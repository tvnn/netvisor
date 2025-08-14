use serde::{Deserialize, Serialize};
use crate::components::tests::types::BaseTestConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConnectivityConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub target: String,     // VPN server endpoint
    pub port: Option<u16>,  // VPN port (default: 51820 for WireGuard)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnTunnelConfig {
    #[serde(flatten)]
    pub base: BaseTestConfig,
    pub expected_subnet: String, // Expected VPN subnet (e.g., "10.100.0.0/24")
}

impl Default for VpnConnectivityConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            target: String::new(),
            port: Some(51820), // WireGuard default
        }
    }
}

impl Default for VpnTunnelConfig {
    fn default() -> Self {
        Self {
            base: BaseTestConfig::default(),
            expected_subnet: "10.100.0.0/24".to_string(),
        }
    }
}