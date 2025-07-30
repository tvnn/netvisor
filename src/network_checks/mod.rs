use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::{CheckConfig, CheckResult};

mod basic;
mod vpn;
mod dns;
mod email;
mod local;
mod services;
mod security;
mod performance;
mod analysis;
mod cdn;
mod utils;

use basic::*;
use vpn::*;
use dns::*;
use email::*;
use local::*;
use services::*;
use security::*;
use performance::*;
use analysis::*;
use cdn::*;

pub use utils::{
    create_http_client,
    get_common_service_name,
};

#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("DNS error: {0}")]
    Dns(#[from] trust_dns_resolver::error::ResolveError),
    // #[error("Timeout")]
    // Timeout,
    #[error("Invalid configuration: {0}")]
    Config(String),
    // #[error("Check failed: {0}")]
    // Failed(String),
}

pub async fn execute_check(check_type: &str, config: &CheckConfig) -> CheckResult {
    let start_time = std::time::Instant::now();
    let start_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    // Call the network_checks function directly - function names match check types exactly
    let result = match check_type {
        "connectivityCheck" => connectivity_check(&config).await,
        "directIpCheck" => direct_ip_check(&config).await,
        "serviceHealthCheck" => service_health_check(&config).await,
        "responseTimeCheck" => response_time_check(&config).await,
        "pingCheck" => ping_check(&config).await,
        "wellknownIpCheck" => wellknown_ip_check(&config).await,
        "dnsResolutionCheck" => dns_resolution_check(&config).await,
        "dnsOverHttpsCheck" => dns_over_https_check(&config).await,
        "vpnConnectivityCheck" => vpn_connectivity_check(&config).await,
        "vpnTunnelCheck" => vpn_tunnel_check(&config).await,
        "localGatewayCheck" => local_gateway_check(&config).await,
        "dhcpDiscoveryCheck" => dhcp_discovery_check(&config).await,
        "subnetScanCheck" => subnet_scan_check(&config).await,
        "smtpCheck" => smtp_check(&config).await,
        "imapCheck" => imap_check(&config).await,
        "pop3Check" => pop3_check(&config).await,
        "sslCertificateCheck" => ssl_certificate_check(&config).await,
        "ftpCheck" => ftp_check(&config).await,
        "sshCheck" => ssh_check(&config).await,
        "databaseCheck" => database_check(&config).await,
        "ntpCheck" => ntp_check(&config).await,
        "ldapCheck" => ldap_check(&config).await,
        "sipCheck" => sip_check(&config).await,
        "bandwidthCheck" => bandwidth_check(&config).await,
        "packetLossCheck" => packet_loss_check(&config).await,
        "jitterCheck" => jitter_check(&config).await,
        "mtuDiscoveryCheck" => mtu_discovery_check(&config).await,
        "tracerouteCheck" => traceroute_check(&config).await,
        "portScanCheck" => port_scan_check(&config).await,
        "cdnCheck" => cdn_check(&config).await,
        _ => Err(CheckError::Config(format!("Unknown check type: {}", check_type)).to_string()),
    };
    
    let duration = start_time.elapsed().as_millis() as u64;
    let end_timestamp = start_timestamp + duration;
    
    match result {
        Ok(details) => CheckResult {
            check_type: check_type.to_string(),
            config: config.clone(),
            success: true,
            message: "Check completed successfully".to_string(),
            error: None,
            details: Some(details),
            duration,
            start_time: start_timestamp,
            end_time: end_timestamp,
        },
        Err(error) => CheckResult {
            check_type: check_type.to_string(),
            config: config.clone(),
            success: false,
            message: format!("Check failed: {}", error),
            error: Some(error.to_string()),
            details: None,
            duration,
            start_time: start_timestamp,
            end_time: end_timestamp,
        }
    }
}