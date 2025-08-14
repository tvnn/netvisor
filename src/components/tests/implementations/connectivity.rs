use anyhow::Result;
use std::time::{Duration, Instant};
use crate::components::tests::types::{TestResult, TestType};
use crate::components::tests::configs::{ConnectivityConfig, DirectIpConfig, PingConfig, WellknownIpConfig};

/// Execute connectivity test with type-safe configuration
pub async fn execute_connectivity_test(config: &ConnectivityConfig) -> Result<TestResult> {
    let start = Instant::now();
    
    let target = &config.target;
    let port = config.port.unwrap_or(80);
    let timeout = Duration::from_millis(config.base.timeout.unwrap_or(30000));
    
    // Attempt to establish TCP connection
    let result = tokio::time::timeout(
        timeout,
        tokio::net::TcpStream::connect(format!("{}:{}", target, port))
    ).await;
    
    let duration = start.elapsed();
    let success = result.is_ok() && result.unwrap().is_ok();
    
    let message = if success {
        format!("Successfully connected to {}:{}", target, port)
    } else {
        format!("Failed to connect to {}:{}", target, port)
    };
    
    Ok(TestResult {
        test_type: TestType::Connectivity,
        success,
        message,
        duration_ms: duration.as_millis() as u64,
        executed_at: chrono::Utc::now(),
        details: Some(serde_json::json!({
            "target": target,
            "port": port,
            "timeout_ms": timeout.as_millis()
        })),
    })
}

/// Execute direct IP test with type-safe configuration
pub async fn execute_direct_ip_test(config: &DirectIpConfig) -> Result<TestResult> {
    let start = Instant::now();
    
    let target = &config.target;
    let port = config.port;
    let timeout = Duration::from_millis(config.base.timeout.unwrap_or(30000));
    
    // Validate IP address format
    if target.parse::<std::net::IpAddr>().is_err() {
        let duration = start.elapsed();
        return Ok(TestResult {
            test_type: TestType::DirectIp,
            success: false,
            message: format!("Invalid IP address format: {}", target),
            duration_ms: duration.as_millis() as u64,
            executed_at: chrono::Utc::now(),
            details: Some(serde_json::json!({
                "target": target,
                "port": port,
                "error": "invalid_ip_format"
            })),
        });
    }
    
    // Attempt to establish TCP connection
    let result = tokio::time::timeout(
        timeout,
        tokio::net::TcpStream::connect(format!("{}:{}", target, port))
    ).await;
    
    let duration = start.elapsed();
    let success = result.is_ok() && result.unwrap().is_ok();
    
    let message = if success {
        format!("Successfully connected to {}:{}", target, port)
    } else {
        format!("Failed to connect to {}:{}", target, port)
    };
    
    Ok(TestResult {
        test_type: TestType::DirectIp,
        success,
        message,
        duration_ms: duration.as_millis() as u64,
        executed_at: chrono::Utc::now(),
        details: Some(serde_json::json!({
            "target": target,
            "port": port,
            "timeout_ms": timeout.as_millis()
        })),
    })
}

/// Execute ping test with type-safe configuration
pub async fn execute_ping_test(config: &PingConfig) -> Result<TestResult> {
    let start = Instant::now();
    
    let target = &config.target;
    let attempts = config.attempts.unwrap_or(4);
    let timeout = Duration::from_millis(config.base.timeout.unwrap_or(30000));
    
    // Use system ping command for now (could be replaced with raw ICMP later)
    let mut successful_pings = 0;
    let mut ping_times = Vec::new();
    
    for _i in 0..attempts {
        let ping_start = Instant::now();
        
        #[cfg(target_os = "windows")]
        let output = tokio::process::Command::new("ping")
            .args(&["-n", "1", target])
            .output()
            .await;
            
        #[cfg(not(target_os = "windows"))]
        let output = tokio::process::Command::new("ping")
            .args(&["-c", "1", target])
            .output()
            .await;
        
        let ping_duration = ping_start.elapsed();
        
        if let Ok(output) = output {
            if output.status.success() {
                successful_pings += 1;
                ping_times.push(ping_duration.as_millis() as u64);
            }
        }
        
        // Break early if we exceed timeout
        if start.elapsed() > timeout {
            break;
        }
    }
    
    let duration = start.elapsed();
    let success = successful_pings > 0;
    let avg_time = if !ping_times.is_empty() {
        ping_times.iter().sum::<u64>() / ping_times.len() as u64
    } else {
        0
    };
    
    let message = if success {
        format!("Ping successful: {}/{} packets, avg {}ms", successful_pings, attempts, avg_time)
    } else {
        format!("Ping failed: 0/{} packets responded", attempts)
    };
    
    Ok(TestResult {
        test_type: TestType::Ping,
        success,
        message,
        duration_ms: duration.as_millis() as u64,
        executed_at: chrono::Utc::now(),
        details: Some(serde_json::json!({
            "target": target,
            "attempts": attempts,
            "successful": successful_pings,
            "ping_times_ms": ping_times,
            "avg_time_ms": avg_time
        })),
    })
}

/// Execute well-known IP test with type-safe configuration
pub async fn execute_wellknown_ip_test(config: &WellknownIpConfig) -> Result<TestResult> {
    let start = Instant::now();
    
    let well_known_ips = vec![
        ("8.8.8.8", "Google DNS"),
        ("1.1.1.1", "Cloudflare DNS"),
        ("208.67.222.222", "OpenDNS"),
    ];
    
    let timeout = Duration::from_millis(config.base.timeout.unwrap_or(30000));
    let mut successful_connections = 0;
    let mut results = Vec::new();
    
    for (ip, name) in &well_known_ips {
        let conn_start = Instant::now();
        
        let result = tokio::time::timeout(
            Duration::from_millis(5000), // 5 second timeout per IP
            tokio::net::TcpStream::connect(format!("{}:53", ip))
        ).await;
        
        let conn_duration = conn_start.elapsed();
        let success = result.is_ok() && result.unwrap().is_ok();
        
        if success {
            successful_connections += 1;
        }
        
        results.push(serde_json::json!({
            "ip": ip,
            "name": name,
            "success": success,
            "duration_ms": conn_duration.as_millis()
        }));
        
        // Break early if we exceed total timeout
        if start.elapsed() > timeout {
            break;
        }
    }
    
    let duration = start.elapsed();
    let success = successful_connections > 0;
    
    let message = if success {
        format!("Connected to {}/{} well-known services", successful_connections, well_known_ips.len())
    } else {
        "Failed to connect to any well-known services".to_string()
    };
    
    Ok(TestResult {
        test_type: TestType::WellknownIp,
        success,
        message,
        duration_ms: duration.as_millis() as u64,
        executed_at: chrono::Utc::now(),
        details: Some(serde_json::json!({
            "results": results,
            "successful_connections": successful_connections,
            "total_tested": well_known_ips.len()
        })),
    })
}