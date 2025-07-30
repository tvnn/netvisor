use crate::types::*;
use crate::network_checks::create_http_client;
use serde_json::{json, Value};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;

// Connectivity test - basic HTTP/HTTPS connection
pub async fn connectivity_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(443);
    let protocol = config.protocol.as_deref().unwrap_or("https");
    let timeout_ms = config.timeout;
    
    let client = create_http_client(timeout_ms)?;
    let url = format!("{}://{}:{}", protocol, target, port);
    
    println!("Testing connectivity to {}", url);
    
    let start = Instant::now();
    let response = client
        .head(&url)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    
    let duration = start.elapsed();
    let status = response.status();
    
    if status.is_success() || status.is_redirection() {
        Ok(json!({
            "url": url,
            "status_code": status.as_u16(),
            "response_time_ms": duration.as_millis(),
            "headers": response.headers().len(),
            "protocol": protocol
        }))
    } else {
        Err(format!("HTTP error: {} {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown")))
    }
}

// Direct IP connectivity test - bypasses DNS entirely
pub async fn direct_ip_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target IP address is required")?;
    let port = config.port.unwrap_or(443);
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    // Validate that target is an IP address, not a domain
    if target.parse::<IpAddr>().is_err() {
        return Err("Target must be an IP address for direct IP test".to_string());
    }
    
    println!("Testing direct IP connectivity to {}:{}", target, port);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.parse::<SocketAddr>()
        .map_err(|e| format!("Invalid socket address: {}", e))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(_stream)) => {
            let duration = start.elapsed();
            Ok(json!({
                "target_ip": target,
                "port": port,
                "response_time_ms": duration.as_millis(),
                "status": "reachable",
                "bypassed_dns": true
            }))
        },
        Ok(Err(e)) => {
            Err(format!("Direct IP connection failed: {}", e))
        },
        Err(_) => {
            Err(format!("Direct IP connection timeout after {}ms", timeout_ms))
        }
    }
}

// Service health test - HTTP status check
pub async fn service_health_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(80);
    let path = config.path.as_deref().unwrap_or("/");
    let expected_status = config.expected_status.unwrap_or(200);
    let timeout_ms = config.timeout;
    
    let client = create_http_client(timeout_ms)?;
    let protocol = if port == 443 { "https" } else { "http" };
    let url = format!("{}://{}:{}{}", protocol, target, port, path);
    
    println!("Testing service health: {}", url);
    
    let start = Instant::now();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Health check failed: {}", e))?;
    
    let duration = start.elapsed();
    let status = response.status().as_u16();
    let content_length = response.content_length().unwrap_or(0);
    
    if status == expected_status {
        Ok(json!({
            "url": url,
            "status_code": status,
            "expected_status": expected_status,
            "response_time_ms": duration.as_millis(),
            "content_length": content_length
        }))
    } else {
        Err(format!("Unexpected status code: expected {}, got {}", expected_status, status))
    }
}

// Response time test - latency measurement
pub async fn response_time_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(443);
    let max_response_time = config.max_response_time.unwrap_or(1000);
    let timeout_ms = config.timeout;
    
    let client = create_http_client(timeout_ms)?;
    let protocol = if port == 443 { "https" } else { "http" };
    let url = format!("{}://{}:{}", protocol, target, port);
    
    println!("Testing response time for {}", url);
    
    let start = Instant::now();
    let response = client
        .head(&url)
        .send()
        .await
        .map_err(|e| format!("Response time test failed: {}", e))?;
    
    let duration = start.elapsed();
    let response_time_ms = duration.as_millis() as u64;
    
    if response.status().is_success() {
        if response_time_ms <= max_response_time {
            Ok(json!({
                "url": url,
                "response_time_ms": response_time_ms,
                "max_response_time_ms": max_response_time,
                "status_code": response.status().as_u16(),
                "within_threshold": true
            }))
        } else {
            Err(format!("Response time {}ms exceeds threshold of {}ms", response_time_ms, max_response_time))
        }
    } else {
        Err(format!("HTTP error: {} {}", response.status().as_u16(), response.status().canonical_reason().unwrap_or("Unknown")))
    }
}

// Ping test - TCP connectivity with multiple attempts
pub async fn ping_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(443);
    let attempts = config.attempts.unwrap_or(3);
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    println!("Testing ping connectivity to {}:{} with {} attempts", target, port, attempts);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for {}", target))?;
    
    let mut successful_attempts = 0;
    let mut response_times = Vec::new();
    let mut errors = Vec::new();
    
    for attempt in 1..=attempts {
        let start = Instant::now();
        let connect_future = TcpStream::connect(&addr);
        
        match timeout(Duration::from_millis(timeout_ms), connect_future).await {
            Ok(Ok(_)) => {
                let duration = start.elapsed();
                successful_attempts += 1;
                response_times.push(duration.as_millis() as u64);
                println!("Ping attempt {}: Success ({}ms)", attempt, duration.as_millis());
            },
            Ok(Err(e)) => {
                errors.push(format!("Attempt {}: {}", attempt, e));
                println!("Ping attempt {}: Failed - {}", attempt, e);
            },
            Err(_) => {
                errors.push(format!("Attempt {}: Timeout", attempt));
                println!("Ping attempt {}: Timeout", attempt);
            }
        }
    }
    
    let success_rate = (successful_attempts as f64 / attempts as f64) * 100.0;
    let avg_response_time = if !response_times.is_empty() {
        response_times.iter().sum::<u64>() / response_times.len() as u64
    } else {
        0
    };
    
    if successful_attempts > 0 {
        Ok(json!({
            "target": format!("{}:{}", target, port),
            "attempts": attempts,
            "successful_attempts": successful_attempts,
            "success_rate_percent": success_rate,
            "avg_response_time_ms": avg_response_time,
            "response_times": response_times,
            "errors": errors
        }))
    } else {
        Err(format!("All {} ping attempts failed", attempts))
    }
}

// Well-known IP test - tests connectivity to major internet infrastructure using IPs
pub async fn wellknown_ip_check(config: &CheckConfig) -> Result<Value, String> {
    let timeout_ms = config.timeout.unwrap_or(3000);
    
    println!("Testing connectivity to well-known IP addresses");
    
    // Well-known IP addresses that should always be reachable
    let wellknown_ips = vec![
        ("8.8.8.8", 53, "Google DNS"),           // Google Public DNS
        ("1.1.1.1", 53, "Cloudflare DNS"),      // Cloudflare DNS
        ("8.8.4.4", 53, "Google DNS Secondary"), // Google Secondary DNS
        ("9.9.9.9", 53, "Quad9 DNS"),           // Quad9 DNS
    ];
    
    let mut successful_tests = Vec::new();
    let mut failed_tests = Vec::new();
    
    for (ip, port, description) in &wellknown_ips {
        let start = Instant::now();
        let socket_addr = format!("{}:{}", ip, port);
        let addr = socket_addr.parse::<SocketAddr>().unwrap();
        
        let connect_future = TcpStream::connect(&addr);
        
        match timeout(Duration::from_millis(timeout_ms), connect_future).await {
            Ok(Ok(_)) => {
                let duration = start.elapsed();
                successful_tests.push(json!({
                    "ip": ip,
                    "port": port,
                    "description": description,
                    "response_time_ms": duration.as_millis(),
                    "status": "reachable"
                }));
            },
            Ok(Err(e)) => {
                failed_tests.push(json!({
                    "ip": ip,
                    "port": port,
                    "description": description,
                    "error": e.to_string(),
                    "status": "unreachable"
                }));
            },
            Err(_) => {
                failed_tests.push(json!({
                    "ip": ip,
                    "port": port,
                    "description": description,
                    "error": "timeout",
                    "status": "timeout"
                }));
            }
        }
    }
    
    if !successful_tests.is_empty() {
        Ok(json!({
            "internet_connectivity": "available",
            "reachable_services": successful_tests,
            "failed_services": failed_tests,
            "total_tested": wellknown_ips.len()
        }))
    } else {
        Err("No well-known IP addresses reachable - internet connectivity may be down".to_string())
    }
}