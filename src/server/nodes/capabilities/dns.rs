use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DnsServiceCapability {}
impl Default for DnsServiceCapability { fn default() -> Self { Self {} } }

// impl<'a> CapabilityWithNode<'a, DnsServiceCapability> {
//     /// Perform forward DNS resolution using this DNS server node
//     pub async fn resolve_domain(
//         &self,
//         domain: &str, 
//         timeout_ms: Option<u32>
//     ) -> Result<DnsResolutionResult> {
//         let timer = Timer::now();
//         let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
        
//         // Get DNS server address and port from node target
//         let (dns_server_addr, dns_server_port) = match &self.node.base.target {
//             NodeTarget::IpAddress(config) => (config.ip, config.port.unwrap_or(53)),
//             NodeTarget::Hostname(..) => {
//                 // For hostname, we need to resolve it first - for now use system resolver
//                 return Err(anyhow::anyhow!("Hostname targets for DNS servers not yet supported"));
//             },
//             NodeTarget::Service(..) => {
//                 // For service, we need to resolve hostname first - for now use system resolver  
//                 return Err(anyhow::anyhow!("Service targets for DNS servers not yet supported"));
//             },
//         };
        
//         let dns_server_string = dns_server_addr.to_string();
        
//         // Create resolver configured to use this specific DNS server
//         let mut resolver_config = ResolverConfig::new();
//         let name_server_config = NameServerConfig::new(
//             SocketAddr::new(dns_server_addr, dns_server_port),
//             Protocol::Udp
//         );
//         resolver_config.add_name_server(name_server_config);
        
//         let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());

//         // Perform DNS resolution with timeout
//         let resolution_result = timeout(timeout_duration, resolver.lookup_ip(domain)).await;

//         let result = match resolution_result {
//             Ok(Ok(lookup)) => {
//                 let resolved_addresses: Vec<IpAddr> = lookup.iter().collect();
                
//                 DnsResolutionResult {
//                     success: true,
//                     resolved_addresses,
//                     duration_ms: timer.elapsed_ms(),
//                     dns_server: format!("{}:{}", dns_server_string, dns_server_port),
//                     error_message: None,
//                 }
//             },
//             Ok(Err(e)) => {
//                 DnsResolutionResult {
//                     success: false,
//                     resolved_addresses: vec![],
//                     duration_ms: timer.elapsed_ms(),
//                     dns_server: format!("{}:{}", dns_server_string, dns_server_port),
//                     error_message: Some(format!("DNS resolution failed: {}", e)),
//                 }
//             },
//             Err(_) => {
//                 DnsResolutionResult {
//                     success: false,
//                     resolved_addresses: vec![],
//                     duration_ms: timer.elapsed_ms(),
//                     dns_server: format!("{}:{}", dns_server_string, dns_server_port),
//                     error_message: Some(format!("DNS resolution timed out after {}ms", timeout_duration.as_millis())),
//                 }
//             }
//         };

//         Ok(result)
//     }

//     /// Perform reverse DNS lookup using this DNS server node
//     pub async fn reverse_lookup_ip(
//         &self,
//         ip_address: IpAddr,
//         timeout_ms: Option<u32>
//     ) -> Result<ReverseDnsResult> {
//         let timer = Timer::now();
//         let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000) as u64);
        
//         // Get DNS server address and port from node target
//         let (dns_server_addr, dns_server_port) = match &self.node.base.target {
//             NodeTarget::IpAddress(config) => (config.ip, config.port.unwrap_or(53)),
//             NodeTarget::Hostname(..) => {
//                 // For hostname, we need to resolve it first - for now use system resolver
//                 return Err(anyhow::anyhow!("Hostname targets for DNS servers not yet supported"));
//             },
//             NodeTarget::Service(..) => {
//                 // For service, we need to resolve hostname first - for now use system resolver  
//                 return Err(anyhow::anyhow!("Service targets for DNS servers not yet supported"));
//             },
//         };
        
//         let dns_server_string = dns_server_addr.to_string();
        
//         // Create resolver configured to use this specific DNS server
//         let mut resolver_config = ResolverConfig::new();
//         let name_server_config = NameServerConfig::new(
//             SocketAddr::new(dns_server_addr, dns_server_port),
//             Protocol::Udp
//         );
//         resolver_config.add_name_server(name_server_config);
        
//         let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());

//         // Perform reverse DNS lookup with timeout
//         let lookup_result = timeout(timeout_duration, resolver.reverse_lookup(ip_address)).await;

//         let result = match lookup_result {
//             Ok(Ok(lookup)) => {
//                 let resolved_domains: Vec<String> = lookup.iter()
//                     .map(|name| name.to_string())
//                     .collect();
                
//                 ReverseDnsResult {
//                     success: true,
//                     resolved_domains,
//                     duration_ms: timer.elapsed_ms(),
//                     dns_server: format!("{}:{}", dns_server_string, dns_server_port),
//                     error_message: None,
//                 }
//             },
//             Ok(Err(e)) => {
//                 ReverseDnsResult {
//                     success: false,
//                     resolved_domains: vec![],
//                     duration_ms: timer.elapsed_ms(),
//                     dns_server: format!("{}:{}", dns_server_string, dns_server_port),
//                     error_message: Some(format!("Reverse DNS lookup failed: {}", e)),
//                 }
//             },
//             Err(_) => {
//                 ReverseDnsResult {
//                     success: false,
//                     resolved_domains: vec![],
//                     duration_ms: timer.elapsed_ms(),
//                     dns_server: format!("{}:{}", dns_server_string, dns_server_port),
//                     error_message: Some(format!("Reverse DNS lookup timed out after {}ms", timeout_duration.as_millis())),
//                 }
//             }
//         };

//         Ok(result)
//     }
// }