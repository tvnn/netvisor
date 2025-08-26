use std::{fmt::{self, Display, Formatter}, net::{IpAddr, SocketAddr}, time::Duration};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use trust_dns_resolver::{TokioAsyncResolver, config::*};
use crate::server::tests::types::execution::Timer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsServerConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub name: String, // For debugging/logging
}

impl Display for DnsServerConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

/// Result structure for DNS resolution operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResolutionResult {
    pub success: bool,
    pub resolved_addresses: Vec<IpAddr>,
    pub duration_ms: u64,
    pub dns_server: DnsServerConfig,
    pub error_message: Option<String>,
}

/// Result structure for reverse DNS lookup operations  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseDnsResult {
    pub success: bool,
    pub resolved_domains: Vec<String>,
    pub duration_ms: u64, 
    pub dns_server: DnsServerConfig,
    pub error_message: Option<String>,
}
pub struct DnsUtils;

impl DnsUtils {
    /// Resolve domain using specific DNS server
    pub async fn resolve_domain(
        dns_server: &DnsServerConfig,
        domain: &str,
        timeout_ms: Option<u32>
    ) -> Result<DnsResolutionResult> {
        let timer = Timer::now();
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
        
        // Create resolver configured to use specific DNS server
        let mut resolver_config = ResolverConfig::new();
        let name_server_config = NameServerConfig::new(
            SocketAddr::new(dns_server.ip, dns_server.port),
            Protocol::Udp
        );
        resolver_config.add_name_server(name_server_config);
        
        let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());
        let lookup_result = timeout(timeout_duration, resolver.lookup_ip(domain)).await;

        let result = match lookup_result {
            Ok(Ok(lookup)) => {
                let resolved_addresses: Vec<IpAddr> = lookup.iter().collect();
                DnsResolutionResult {
                    success: true,
                    resolved_addresses,
                    duration_ms: timer.elapsed_ms(),
                    dns_server: dns_server.clone(),
                    error_message: None,
                }
            },
            Ok(Err(e)) => {
                DnsResolutionResult {
                    success: false,
                    resolved_addresses: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server: dns_server.clone(),
                    error_message: Some(format!("DNS resolution failed: {}", e)),
                }
            },
            Err(_) => {
                DnsResolutionResult {
                    success: false,
                    resolved_addresses: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server: dns_server.clone(),
                    error_message: Some(format!("DNS resolution timed out after {}ms", timeout_duration.as_millis())),
                }
            }
        };

        Ok(result)
    }

    /// Perform reverse DNS lookup using specific DNS server
    pub async fn reverse_lookup_ip(
        dns_server: &DnsServerConfig,
        ip_address: IpAddr,
        timeout_ms: Option<u32>
    ) -> Result<ReverseDnsResult> {
        let timer = Timer::now();
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
        
        let mut resolver_config = ResolverConfig::new();
        let name_server_config = NameServerConfig::new(
            SocketAddr::new(dns_server.ip, dns_server.port),
            Protocol::Udp
        );
        resolver_config.add_name_server(name_server_config);
        
        let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());
        let lookup_result = timeout(timeout_duration, resolver.reverse_lookup(ip_address)).await;

        let result = match lookup_result {
            Ok(Ok(lookup)) => {
                let resolved_domains: Vec<String> = lookup.iter()
                    .map(|name| name.to_string())
                    .collect();
                
                ReverseDnsResult {
                    success: true,
                    resolved_domains,
                    duration_ms: timer.elapsed_ms(),
                    dns_server: dns_server.clone(),
                    error_message: None,
                }
            },
            Ok(Err(e)) => {
                ReverseDnsResult {
                    success: false,
                    resolved_domains: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server: dns_server.clone(),
                    error_message: Some(format!("Reverse DNS lookup failed: {}", e)),
                }
            },
            Err(_) => {
                ReverseDnsResult {
                    success: false,
                    resolved_domains: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server: dns_server.clone(),
                    error_message: Some("Reverse DNS lookup timed out".to_string()),
                }
            }
        };

        Ok(result)
    }

    // /// Resolve hostname to IP using system resolver (for general connectivity)
    // pub async fn resolve_hostname_system(
    //     hostname: &str,
    //     timeout_ms: Option<u32>
    // ) -> Result<IpAddr> {
    //     let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
    //     let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    //     let lookup_result = timeout(timeout_duration, resolver.lookup_ip(hostname)).await;
        
    //     match lookup_result {
    //         Ok(Ok(lookup)) => {
    //             lookup.iter().next()
    //                 .ok_or_else(|| anyhow::anyhow!("No IP addresses found for hostname: {}", hostname))
    //         },
    //         Ok(Err(e)) => Err(anyhow::anyhow!("Hostname resolution failed: {}", e)),
    //         Err(_) => Err(anyhow::anyhow!("Hostname resolution timed out")),
    //     }
    // }
}
