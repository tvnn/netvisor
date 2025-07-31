// // src/discovery/scanner.rs
// use super::types::*;
// use std::collections::{HashMap, HashSet};
// use std::net::{IpAddr, Ipv4Addr};
// use std::sync::Arc;
// use std::time::{Duration, Instant};
// use tokio::net::TcpStream;
// use tokio::sync::mpsc;
// use tokio::time::timeout;

// pub async fn ping_host(ip: &str, timeout_ms: u64) -> Result<bool, Box<dyn std::error::Error>> {
//     // Use TCP connect to port 80 as a proxy for "ping" since ICMP requires privileges
//     let addr = format!("{}:80", ip);
//     match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
//         Ok(Ok(_)) => Ok(true),
//         _ => {
//             // Try port 22 as fallback
//             let addr = format!("{}:22", ip);
//             match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
//                 Ok(Ok(_)) => Ok(true),
//                 _ => Ok(false)
//             }
//         }
//     }
// }

// pub async fn test_port(ip: &str, port: u16, timeout_ms: u64) -> Result<bool, Box<dyn std::error::Error>> {
//     let addr = format!("{}:{}", ip, port);
//     match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
//         Ok(Ok(_)) => Ok(true),
//         _ => Ok(false)
//     }
// }

// pub async fn ping_sweep(
//     target_ips: &[String], 
//     config: &DiscoveryConfig,
//     stop_signal: Arc<tokio::sync::RwLock<bool>>
// ) -> Vec<String> {
//     let (tx, mut rx) = mpsc::channel(100);
//     let mut alive_hosts = Vec::new();
//     let semaphore = Arc::new(tokio::sync::Semaphore::new(config.max_concurrent));

//     // Spawn tasks for each IP
//     for ip in target_ips {
//         let ip = ip.clone();
//         let tx = tx.clone();
//         let semaphore = semaphore.clone();
//         let timeout_ms = config.timeout_ms;
//         let stop_signal = stop_signal.clone();

//         tokio::spawn(async move {
//             let _permit = semaphore.acquire().await.unwrap();
            
//             if *stop_signal.read().await {
//                 return;
//             }

//             if let Ok(true) = ping_host(&ip, timeout_ms).await {
//                 let _ = tx.send(ip).await;
//             }
//         });
//     }

//     drop(tx); // Close the channel

//     // Collect results
//     while let Some(ip) = rx.recv().await {
//         alive_hosts.push(ip);
        
//         if *stop_signal.read().await {
//             break;
//         }
//     }

//     alive_hosts
// }

// pub async fn port_scan(
//     hosts: &[String], 
//     config: &DiscoveryConfig,
//     stop_signal: Arc<tokio::sync::RwLock<bool>>
// ) -> HashMap<String, Vec<u16>> {
//     let mut results = HashMap::new();
//     let semaphore = Arc::new(tokio::sync::Semaphore::new(config.max_concurrent));

//     for host in hosts {
//         if *stop_signal.read().await {
//             break;
//         }

//         let mut open_ports = Vec::new();
        
//         for &port in &config.common_ports {
//             let _permit = semaphore.acquire().await.unwrap();
            
//             if *stop_signal.read().await {
//                 break;
//             }

//             if let Ok(true) = test_port(host, port, config.timeout_ms).await {
//                 open_ports.push(port);
//             }
//         }

//         if !open_ports.is_empty() {
//             results.insert(host.clone(), open_ports);
//         }
//     }

//     results
// }

// pub fn classify_device_type(ports: &[u16]) -> DeviceType {
//     let port_set: HashSet<u16> = ports.iter().cloned().collect();
    
//     // Router/Switch indicators
//     if port_set.contains(&161) && (port_set.contains(&22) || port_set.contains(&23)) {
//         return DeviceType::Router;
//     }
    
//     // Server indicators
//     if port_set.contains(&22) && (port_set.contains(&80) || port_set.contains(&443)) {
//         return DeviceType::Server;
//     }
    
//     // Printer indicators
//     if port_set.contains(&631) || port_set.contains(&9100) {
//         return DeviceType::Printer;
//     }
    
//     // NAS indicators
//     if port_set.contains(&139) && port_set.contains(&445) {
//         return DeviceType::NAS;
//     }
    
//     // IoT indicators (common IoT ports)
//     if port_set.contains(&8080) || port_set.contains(&8443) {
//         return DeviceType::IoT;
//     }
    
//     // Access Point indicators
//     if port_set.contains(&80) && port_set.contains(&443) && ports.len() <= 3 {
//         return DeviceType::AccessPoint;
//     }
    
//     DeviceType::Unknown
// }

// pub async fn detect_services_on_ports(ip: &str, ports: &[u16], timeout_ms: u64) -> Vec<String> {
//     let mut services = Vec::new();
    
//     for &port in ports {
//         let service = match port {
//             22 => "SSH",
//             23 => "Telnet", 
//             25 => "SMTP",
//             53 => "DNS",
//             80 => "HTTP",
//             110 => "POP3",
//             143 => "IMAP",
//             443 => "HTTPS",
//             993 => "IMAPS",
//             995 => "POP3S",
//             3389 => "RDP",
//             5900 => "VNC",
//             8080 => "HTTP-Alt",
//             8443 => "HTTPS-Alt",
//             _ => continue,
//         };
        
//         services.push(service.to_string());
//     }
    
//     services
// }