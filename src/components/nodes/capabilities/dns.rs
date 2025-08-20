use std::{net::IpAddr, time::Duration};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use trust_dns_resolver::{TokioAsyncResolver, config::*};
use crate::components::{
    nodes::types::{base::{CapabilityWithNode, Node}, targets::NodeTarget},
    tests::types::execution::Timer,
};

/// Result structure for DNS resolution operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResolutionResult {
    pub success: bool,
    pub resolved_addresses: Vec<IpAddr>,
    pub duration_ms: u64,
    pub dns_server: String,
    pub error_message: Option<String>,
}

/// Result structure for reverse DNS lookup operations  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseDnsResult {
    pub success: bool,
    pub resolved_domains: Vec<String>,
    pub duration_ms: u64, 
    pub dns_server: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DnsServiceCapability {}
impl Default for DnsServiceCapability { fn default() -> Self { Self {} } }

impl<'a> CapabilityWithNode<'a, DnsServiceCapability> {
    /// Perform forward DNS resolution using the specified DNS server node
    pub async fn resolve_domain(
        &self,
        dns_server_node: &Node,
        domain: &str, 
        timeout_ms: Option<u32>
    ) -> Result<DnsResolutionResult> {
        let timer = Timer::now();
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
        
        // Get DNS server address from node configuration
        let dns_server = Self::get_dns_server_address(dns_server_node);
        
        // Create resolver directly - TODO: configure to use specific DNS server
        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        );

        // Perform DNS resolution with timeout
        let resolution_result = timeout(
            timeout_duration,
            resolver.lookup_ip(domain)
        ).await;

        let result = match resolution_result {
            Ok(Ok(lookup)) => {
                let resolved_addresses: Vec<IpAddr> = lookup.iter().collect();
                
                DnsResolutionResult {
                    success: true,
                    resolved_addresses,
                    duration_ms: timer.elapsed_ms(),
                    dns_server,
                    error_message: None,
                }
            },
            Ok(Err(e)) => {
                DnsResolutionResult {
                    success: false,
                    resolved_addresses: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server,
                    error_message: Some(format!("DNS resolution failed: {}", e)),
                }
            },
            Err(_) => {
                DnsResolutionResult {
                    success: false,
                    resolved_addresses: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server,
                    error_message: Some(format!("DNS resolution timed out after {}ms", timeout_duration.as_millis())),
                }
            }
        };

        Ok(result)
    }

    /// Perform reverse DNS lookup using the specified DNS server node
    pub async fn reverse_lookup_ip(
        &self,
        dns_server_node: &Node,
        ip_address: IpAddr,
        timeout_ms: Option<u32>
    ) -> Result<ReverseDnsResult> {
        let timer = Timer::now();
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
        
        // Get DNS server address from node configuration
        let dns_server = Self::get_dns_server_address(dns_server_node);
        
        // Create resolver directly - TODO: configure to use specific DNS server
        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        );

        // Perform reverse DNS lookup with timeout
        let lookup_result = timeout(
            timeout_duration,
            resolver.reverse_lookup(ip_address)
        ).await;

        let result = match lookup_result {
            Ok(Ok(lookup)) => {
                let resolved_domains: Vec<String> = lookup.iter()
                    .map(|name| name.to_string())
                    .collect();
                
                ReverseDnsResult {
                    success: true,
                    resolved_domains,
                    duration_ms: timer.elapsed_ms(),
                    dns_server,
                    error_message: None,
                }
            },
            Ok(Err(e)) => {
                ReverseDnsResult {
                    success: false,
                    resolved_domains: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server,
                    error_message: Some(format!("Reverse DNS lookup failed: {}", e)),
                }
            },
            Err(_) => {
                ReverseDnsResult {
                    success: false,
                    resolved_domains: vec![],
                    duration_ms: timer.elapsed_ms(),
                    dns_server,
                    error_message: Some(format!("Reverse DNS lookup timed out after {}ms", timeout_duration.as_millis())),
                }
            }
        };

        Ok(result)
    }

    /// Helper method to get DNS server address from node target
    fn get_dns_server_address(dns_server_node: &Node) -> String {
        match &dns_server_node.base.target {
            NodeTarget::IpAddress(config) => config.ip.to_string(),
            NodeTarget::Hostname(config) => config.hostname.clone(),
            NodeTarget::Service(config) => config.hostname.clone(),
        }
    }
}