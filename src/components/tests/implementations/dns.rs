use anyhow::Result;
use std::time::{Duration, Instant};
use crate::components::tests::types::{TestResult, TestType};
use crate::components::tests::configs::{DnsResolutionConfig, DnsOverHttpsConfig};

/// Execute DNS resolution test
pub async fn execute_dns_resolution_test(config: &DnsResolutionConfig) -> Result<TestResult> {
    let start = Instant::now();
    
    let domain = &config.domain;
    let timeout = Duration::from_millis(config.base.timeout.unwrap_or(30000));
    
    // Perform DNS lookup using tokio's built-in resolver
    let result = tokio::time::timeout(
        timeout,
        tokio::net::lookup_host(format!("{}:80", domain))
    ).await;
    
    let duration = start.elapsed();
    
    let (success, message, details) = match result {
        Ok(Ok(addresses)) => {
            let ips: Vec<String> = addresses
                .map(|addr| addr.ip().to_string())
                .collect();
            
            if ips.is_empty() {
                (false, format!("No IP addresses found for {}", domain), serde_json::json!({
                    "domain": domain,
                    "resolved_ips": [],
                    "error": "no_addresses_found"
                }))
            } else {
                (true, format!("Resolved {} to: {}", domain, ips.join(", ")), serde_json::json!({
                    "domain": domain,
                    "resolved_ips": ips,
                    "ip_count": ips.len()
                }))
            }
        },
        Ok(Err(e)) => {
            (false, format!("DNS resolution failed for {}: {}", domain, e), serde_json::json!({
                "domain": domain,
                "error": "resolution_failed",
                "error_details": e.to_string()
            }))
        },
        Err(_) => {
            (false, format!("DNS resolution timed out for {}", domain), serde_json::json!({
                "domain": domain,
                "error": "timeout",
                "timeout_ms": timeout.as_millis()
            }))
        }
    };
    
    Ok(TestResult {
        test_type: TestType::DnsResolution,
        success,
        message,
        duration_ms: duration.as_millis() as u64,
        executed_at: chrono::Utc::now(),
        details: Some(details),
    })
}

/// Execute DNS over HTTPS test (simplified implementation)
pub async fn execute_dns_over_https_test(config: &DnsOverHttpsConfig) -> Result<TestResult> {
    let start = Instant::now();
    
    let target = &config.target;
    let domain = &config.domain;
    let timeout = Duration::from_millis(config.base.timeout.unwrap_or(30000));
    
    // Create a simple DoH query (this is a basic implementation)
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()?;
    
    // Use Cloudflare's DoH JSON API format
    let url = if target.contains("1.1.1.1") {
        format!("https://1.1.1.1/dns-query?name={}&type=A", domain)
    } else if target.contains("8.8.8.8") {
        format!("https://dns.google/resolve?name={}&type=A", domain)
    } else {
        // Generic DoH endpoint
        format!("{}?name={}&type=A", target, domain)
    };
    
    let result = client
        .get(&url)
        .header("Accept", "application/dns-json")
        .send()
        .await;
    
    let duration = start.elapsed();
    
    let (success, message, details) = match result {
        Ok(response) if response.status().is_success() => {
            match response.text().await {
                Ok(body) => {
                    // Try to parse as JSON to extract IPs
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                        if let Some(answers) = json.get("Answer").and_then(|a| a.as_array()) {
                            let ips: Vec<String> = answers
                                .iter()
                                .filter_map(|answer| answer.get("data"))
                                .filter_map(|data| data.as_str())
                                .map(|s| s.to_string())
                                .collect();
                            
                            if ips.is_empty() {
                                (false, format!("No A records found for {} via DoH", domain), serde_json::json!({
                                    "domain": domain,
                                    "doh_endpoint": target,
                                    "resolved_ips": [],
                                    "raw_response": body
                                }))
                            } else {
                                (true, format!("Resolved {} via DoH to: {}", domain, ips.join(", ")), serde_json::json!({
                                    "domain": domain,
                                    "doh_endpoint": target,
                                    "resolved_ips": ips,
                                    "ip_count": ips.len()
                                }))
                            }
                        } else {
                            (false, format!("Invalid DoH response format for {}", domain), serde_json::json!({
                                "domain": domain,
                                "doh_endpoint": target,
                                "error": "invalid_response_format",
                                "raw_response": body
                            }))
                        }
                    } else {
                        // Non-JSON response, treat as basic success if we got a response
                        (true, format!("DoH query for {} returned data", domain), serde_json::json!({
                            "domain": domain,
                            "doh_endpoint": target,
                            "response_length": body.len()
                        }))
                    }
                },
                Err(e) => {
                    (false, format!("Failed to read DoH response for {}: {}", domain, e), serde_json::json!({
                        "domain": domain,
                        "doh_endpoint": target,
                        "error": "response_read_failed",
                        "error_details": e.to_string()
                    }))
                }
            }
        },
        Ok(response) => {
            (false, format!("DoH query failed for {} (HTTP {})", domain, response.status()), serde_json::json!({
                "domain": domain,
                "doh_endpoint": target,
                "error": "http_error",
                "status_code": response.status().as_u16()
            }))
        },
        Err(e) => {
            (false, format!("DoH request failed for {}: {}", domain, e), serde_json::json!({
                "domain": domain,
                "doh_endpoint": target,
                "error": "request_failed",
                "error_details": e.to_string()
            }))
        }
    };
    
    Ok(TestResult {
        test_type: TestType::DnsOverHttps,
        success,
        message,
        duration_ms: duration.as_millis() as u64,
        executed_at: chrono::Utc::now(),
        details: Some(details),
    })
}