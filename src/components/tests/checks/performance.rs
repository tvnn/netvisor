use serde_json::{json, Value};
use super::{create_http_client};
use std::net::{ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use futures::StreamExt;
use crate::components::tests::checks::CheckConfig;

pub async fn bandwidth_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target URL is required")?;
    let test_duration = config.test_duration.unwrap_or(10);
    let test_type = config.test_type.as_deref().unwrap_or("download");
    let timeout_ms = config.timeout.unwrap_or(30000);
    
    println!("Testing {} bandwidth to {} for {} seconds", test_type, target, test_duration);
    
    let client = create_http_client(Some(timeout_ms))?;
    let start = Instant::now();
    
    match test_type {
        "download" => {
            let response = client.get(target)
                .send()
                .await
                .map_err(|e| format!("Bandwidth test request failed: {}", e))?;
            
            if !response.status().is_success() {
                return Err(format!("Bandwidth test HTTP error: {}", response.status()));
            }
            
            let mut total_bytes = 0u64;
            let mut bytes_stream = response.bytes_stream();
            let test_start = Instant::now();
            let max_duration = Duration::from_secs(test_duration.into());
            
            while test_start.elapsed() < max_duration {
                match timeout(Duration::from_millis(1000), bytes_stream.next()).await {
                    Ok(Some(Ok(chunk))) => {
                        total_bytes += chunk.len() as u64;
                    },
                    Ok(Some(Err(e))) => {
                        return Err(format!("Stream error during bandwidth test: {}", e));
                    },
                    Ok(None) => break, // Stream ended
                    Err(_) => break, // Timeout
                }
            }
            
            let actual_duration = test_start.elapsed();
            let duration_secs = actual_duration.as_secs_f64();
            let bytes_per_second = if duration_secs > 0.0 { total_bytes as f64 / duration_secs } else { 0.0 };
            let mbps = (bytes_per_second * 8.0) / 1_000_000.0; // Convert to Mbps
            
            Ok(json!({
                "target": target,
                "test_type": test_type,
                "duration_seconds": duration_secs,
                "total_bytes": total_bytes,
                "bytes_per_second": bytes_per_second,
                "mbps": mbps,
                "response_time_ms": start.elapsed().as_millis(),
                "status": "completed"
            }))
        },
        "upload" => {
            // For upload test, we'd need a server that accepts POST data
            // This is a simplified version that measures connection establishment
            let response = client.post(target)
                .body("bandwidth_test_data")
                .send()
                .await
                .map_err(|e| format!("Upload bandwidth test failed: {}", e))?;
            
            let duration = start.elapsed();
            
            Ok(json!({
                "target": target,
                "test_type": test_type,
                "duration_ms": duration.as_millis(),
                "status_code": response.status().as_u16(),
                "note": "Upload test limited - requires specialized server endpoint",
                "status": "completed"
            }))
        },
        _ => Err("Invalid test type. Use 'download' or 'upload'".to_string()),
    }
}

pub async fn packet_loss_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(443);
    let packet_count = config.packet_count.unwrap_or(20).min(100); // Limit max packets
    let interval_ms = config.interval_ms.unwrap_or(100).max(50); // Minimum 50ms interval
    let timeout_ms = config.timeout.unwrap_or(1000);
    
    println!("Testing packet loss to {}:{} with {} packets", target, port, packet_count);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve target {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for target {}", target))?;
    
    let mut successful_packets = 0;
    let mut failed_packets = 0;
    let mut response_times = Vec::new();
    let mut packet_results = Vec::new();
    
    let test_start = Instant::now();
    
    for packet_num in 1..=packet_count {
        let packet_start = Instant::now();
        
        match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => {
                let rtt = packet_start.elapsed().as_millis() as u64;
                successful_packets += 1;
                response_times.push(rtt);
                packet_results.push(json!({
                    "packet": packet_num,
                    "success": true,
                    "rtt_ms": rtt
                }));
            },
            _ => {
                failed_packets += 1;
                packet_results.push(json!({
                    "packet": packet_num,
                    "success": false,
                    "rtt_ms": null
                }));
            }
        }
        
        // Wait before next packet (except for last packet)
        if packet_num < packet_count {
            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }
    }
    
    let total_duration = test_start.elapsed();
    let packet_loss_percent = (failed_packets as f64 / packet_count as f64) * 100.0;
    let avg_rtt = if !response_times.is_empty() {
        response_times.iter().sum::<u64>() / response_times.len() as u64
    } else {
        0
    };
    
    let min_rtt = response_times.iter().min().copied().unwrap_or(0);
    let max_rtt = response_times.iter().max().copied().unwrap_or(0);
    
    Ok(json!({
        "target": format!("{}:{}", target, port),
        "packet_count": packet_count,
        "successful_packets": successful_packets,
        "failed_packets": failed_packets,
        "packet_loss_percent": packet_loss_percent,
        "avg_rtt_ms": avg_rtt,
        "min_rtt_ms": min_rtt,
        "max_rtt_ms": max_rtt,
        "total_duration_ms": total_duration.as_millis(),
        "interval_ms": interval_ms,
        "packet_results": packet_results,
        "status": if packet_loss_percent == 0.0 { "no_loss" } else if packet_loss_percent < 5.0 { "acceptable" } else { "high_loss" }
    }))
}

pub async fn jitter_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(443);
    let sample_count = config.sample_count.unwrap_or(10).min(50); // Limit samples
    let interval_ms = config.interval_ms.unwrap_or(500).max(100); // Minimum 100ms interval
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    println!("Testing network jitter to {}:{} with {} samples", target, port, sample_count);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve target {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for target {}", target))?;
    
    let mut response_times = Vec::new();
    let mut sample_results = Vec::new();
    
    for sample_num in 1..=sample_count {
        let start = Instant::now();
        
        match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => {
                let rtt = start.elapsed().as_millis() as u64;
                response_times.push(rtt);
                sample_results.push(json!({
                    "sample": sample_num,
                    "rtt_ms": rtt,
                    "success": true
                }));
            },
            _ => {
                sample_results.push(json!({
                    "sample": sample_num,
                    "rtt_ms": null,
                    "success": false
                }));
            }
        }
        
        // Wait before next sample
        if sample_num < sample_count {
            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }
    }
    
    if response_times.is_empty() {
        return Err("No successful connections for jitter measurement".to_string());
    }
    
    // Calculate jitter statistics
    let avg_rtt = response_times.iter().sum::<u64>() as f64 / response_times.len() as f64;
    let min_rtt = *response_times.iter().min().unwrap();
    let max_rtt = *response_times.iter().max().unwrap();
    
    // Calculate jitter (average deviation from mean)
    let mut deviations = Vec::new();
    for i in 1..response_times.len() {
        let diff = (response_times[i] as i64 - response_times[i-1] as i64).abs() as u64;
        deviations.push(diff);
    }
    
    let avg_jitter = if !deviations.is_empty() {
        deviations.iter().sum::<u64>() as f64 / deviations.len() as f64
    } else {
        0.0
    };
    
    let max_jitter = deviations.iter().max().copied().unwrap_or(0);
    
    // Calculate variance and standard deviation
    let variance = response_times.iter()
        .map(|&rtt| {
            let diff = rtt as f64 - avg_rtt;
            diff * diff
        })
        .sum::<f64>() / response_times.len() as f64;
    
    let std_deviation = variance.sqrt();
    
    Ok(json!({
        "target": format!("{}:{}", target, port),
        "sample_count": sample_count,
        "successful_samples": response_times.len(),
        "avg_rtt_ms": avg_rtt,
        "min_rtt_ms": min_rtt,
        "max_rtt_ms": max_rtt,
        "avg_jitter_ms": avg_jitter,
        "max_jitter_ms": max_jitter,
        "std_deviation_ms": std_deviation,
        "variance": variance,
        "sample_results": sample_results,
        "status": if avg_jitter < 10.0 { "low_jitter" } else if avg_jitter < 50.0 { "moderate_jitter" } else { "high_jitter" }
    }))
}
