use crate::types::*;
use crate::network_checks::{create_http_client};
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use tokio::time::timeout;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

// DNS resolution test
pub async fn dns_resolution_check(config: &CheckConfig) -> Result<Value, String> {
    let domain = config.domain.as_ref().ok_or("Domain is required")?;
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    println!("Testing DNS resolution for {}", domain);
    
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );
    
    let start = Instant::now();
    let lookup_future = resolver.lookup_ip(domain);
    
    let lookup_result = timeout(Duration::from_millis(timeout_ms), lookup_future)
        .await
        .map_err(|_| "DNS resolution timeout")?
        .map_err(|e| format!("DNS resolution failed: {}", e))?;
    
    let duration = start.elapsed();
    let resolved_ips: Vec<String> = lookup_result.iter().map(|ip| ip.to_string()).collect();
    
    if !resolved_ips.is_empty() {
        Ok(json!({
            "domain": domain,
            "resolved_ips": resolved_ips,
            "response_time_ms": duration.as_millis(),
            "resolver": "system_default"
        }))
    } else {
        Err(format!("No IP addresses resolved for domain: {}", domain))
    }
}

// DNS over HTTPS test
pub async fn dns_over_https_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("DoH server required")?;
    let default_test_domain = "google.com".to_string();
    let default_service_type = "auto".to_string();
    let test_domain = config.test_domain.as_ref().unwrap_or(&default_test_domain);
    let service_type = config.service_type.as_ref().unwrap_or(&default_service_type);
    let timeout_ms = config.timeout;
    
    // Build DoH URL based on service type
    let doh_url = match service_type.as_str() {
        "google" => "https://dns.google/resolve".to_string(),
        "cloudflare" => "https://cloudflare-dns.com/dns-query".to_string(),
        "pihole" | "custom" => format!("{}/dns-query", target),
        "auto" => {
            if target.contains("google") {
                "https://dns.google/resolve".to_string()
            } else if target.contains("cloudflare") || target.contains("1.1.1.1") {
                "https://cloudflare-dns.com/dns-query".to_string()
            } else {
                format!("{}/dns-query", target)
            }
        },
        _ => format!("{}/dns-query", target),
    };
    
    println!("Testing DNS over HTTPS: {} for domain {}", doh_url, test_domain);
    
    let client = create_http_client(timeout_ms)?;
    let start = Instant::now();
    
    let response = client
        .get(&doh_url)
        .query(&[("name", test_domain.as_str()), ("type", "A")])
        .header("Accept", "application/dns-json")
        .send()
        .await
        .map_err(|e| format!("DoH request failed: {}", e))?;
    
    let duration = start.elapsed();
    let status = response.status();
    
    if !status.is_success() {
        return Err(format!("DoH server error: {} {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown")));
    }
    
    let dns_response: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse DoH response: {}", e))?;
    
    // Extract answers from DNS response
    let answers = dns_response["Answer"].as_array().unwrap_or(&vec![]).len();
    let status_code = dns_response["Status"].as_u64().unwrap_or(999);
    
    if status_code == 0 && answers > 0 {
        Ok(json!({
            "doh_server": doh_url,
            "test_domain": test_domain,
            "response_time_ms": duration.as_millis(),
            "dns_status": status_code,
            "answer_count": answers,
            "service_type": service_type
        }))
    } else {
        Err(format!("DNS resolution failed: status={}, answers={}", status_code, answers))
    }
}