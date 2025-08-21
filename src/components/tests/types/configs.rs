use std::{net::{IpAddr, Ipv4Addr}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cidr::{IpCidr, Ipv4Cidr};


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ConnectivityConfig {
    pub timeout_ms: Option<u32>,
    pub dns_resolver_node: Option<Uuid>
}

impl Default for ConnectivityConfig {
    fn default() -> Self {
        Self {
            timeout_ms: Some(30000),
            dns_resolver_node: Some(Uuid::new_v4())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ServiceHealthConfig {
    pub expected_status_code: u16,
    pub timeout_ms: Option<u32>,
}

impl Default for ServiceHealthConfig {
    fn default() -> Self {
        Self {
            expected_status_code: 200,
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DnsResolutionConfig {
    pub domain: String,                    // Domain to resolve
    pub expected_ip: IpAddr,         // Expected resolution results
    pub timeout_ms: Option<u32>,
}

impl Default for DnsResolutionConfig {
    fn default() -> Self {
        Self {
            domain: "example.com".to_string(),
            expected_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DnsOverHttpsConfig {
    pub domain: String,                    // Domain to resolve via DoH
    pub expected_ip: IpAddr,         // Expected resolution results
    pub timeout_ms: Option<u32>,
}

impl Default for DnsOverHttpsConfig {
    fn default() -> Self {
        Self {
            domain: "example.com".to_string(),
            expected_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DnsLookupConfig {
    pub expected_ip: IpAddr,
    pub timeout_ms: Option<u32>,
    pub dns_resolver_node: Uuid
}

impl Default for DnsLookupConfig {
    fn default() -> Self {
        Self {
            expected_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            timeout_ms: Some(30000),
            dns_resolver_node: Uuid::new_v4()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ReverseDnsConfig {
    pub expected_domain: String,
    pub timeout_ms: Option<u32>,
    pub dns_resolver_node: Uuid
}

impl Default for ReverseDnsConfig {
    fn default() -> Self {
        Self {
            expected_domain: "example.com".to_string(),
            timeout_ms: Some(30000),
            dns_resolver_node: Uuid::new_v4()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct VpnSubnetAccessConfig {
    pub expected_subnet: IpCidr,
    pub timeout_ms: Option<u32>,
    pub dns_resolver_node: Option<Uuid>
}

impl Default for VpnSubnetAccessConfig {
    fn default() -> Self {
        Self {
            expected_subnet: IpCidr::V4(Ipv4Cidr::new(
                Ipv4Addr::new(10, 100, 0, 0), 
                24
            ).expect("Default value should be valid IP")),
            timeout_ms: Some(30000),
            dns_resolver_node: Some(Uuid::new_v4())
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
// pub struct NtpSyncConfig {
//     pub max_offset_ms: u32,               // Maximum acceptable time offset
//     pub timeout_ms: Option<u32>,
// }

// impl Default for NtpSyncConfig {
//     fn default() -> Self {
//         Self {
//             timeout_ms: Some(30000),
//             max_offset_ms: 1000
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
// pub struct DaemonCommandConfig {
//     pub command: String,                   // Command to execute
//     pub expected_output: Option<String>,   // Expected command output
//     pub requires_confirmation: Option<bool>, // User confirmation required
//     pub rollback_command: Option<String>,  // Rollback command if needed
//     pub timeout_ms: Option<u32>,
//     // Executes on the node via installed daemon
// }

// #[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
// pub struct SshScriptConfig {
//     pub command: String,                   // Command to execute via SSH
//     pub ssh_user: Option<String>,          // SSH username (default from node config)
//     pub expected_output: Option<String>,   // Expected command output
//     pub requires_confirmation: Option<bool>, // User confirmation required
//     pub rollback_command: Option<String>,  // Rollback command if needed
//     pub timeout_ms: Option<u32>,
//     // Executes on the node via SSH connection
// }