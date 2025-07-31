use super::{create_http_client};
use serde_json::{json, Value};
use std::time::{Instant};
use std::collections::HashMap;
use crate::components::tests::checks::CheckConfig;

pub async fn cdn_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target URL is required")?;
    let expected_region = config.expected_region.as_deref().unwrap_or("auto");
    let check_headers = config.check_headers.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing CDN performance for {}", target);
    
    let client = create_http_client(Some(timeout_ms))?;
    let start = Instant::now();
    
    let response = client.get(target)
        .send()
        .await
        .map_err(|e| format!("CDN test request failed: {}", e))?;
    
    let duration = start.elapsed();
    let headers = response.headers();
    
    // Extract CDN-related headers
    let mut cdn_info = HashMap::new();
    let mut cache_info = HashMap::new();
    
    if check_headers {
        // Common CDN headers
        if let Some(cf_ray) = headers.get("cf-ray") {
            cdn_info.insert("provider", "Cloudflare");
            cdn_info.insert("edge_id", cf_ray.to_str().unwrap_or(""));
        }
        
        if let Some(x_cache) = headers.get("x-cache") {
            cache_info.insert("status", x_cache.to_str().unwrap_or(""));
        }
        
        if let Some(server) = headers.get("server") {
            cdn_info.insert("server", server.to_str().unwrap_or(""));
        }
        
        if let Some(x_served_by) = headers.get("x-served-by") {
            cdn_info.insert("served_by", x_served_by.to_str().unwrap_or(""));
        }
        
        if let Some(x_cache_status) = headers.get("x-cache-status") {
            cache_info.insert("cache_status", x_cache_status.to_str().unwrap_or(""));
        }
        
        if let Some(age) = headers.get("age") {
            cache_info.insert("age_seconds", age.to_str().unwrap_or(""));
        }
        
        // AWS CloudFront headers
        if let Some(cf_id) = headers.get("x-amz-cf-id") {
            cdn_info.insert("provider", "AWS CloudFront");
            cdn_info.insert("cf_id", cf_id.to_str().unwrap_or(""));
        }
        
        // Azure CDN headers
        if let Some(azure_edge) = headers.get("x-azure-ref") {
            cdn_info.insert("provider", "Azure CDN");
            cdn_info.insert("azure_ref", azure_edge.to_str().unwrap_or(""));
        }
        
        // Fastly headers
        if let Some(fastly_id) = headers.get("fastly-debug-digest") {
            cdn_info.insert("provider", "Fastly");
            cdn_info.insert("fastly_id", fastly_id.to_str().unwrap_or(""));
        }
    }
    
    let status_code = response.status().as_u16();
    let content_length = response.content_length().unwrap_or(0);
    
    Ok(json!({
        "target": target,
        "response_time_ms": duration.as_millis(),
        "status_code": status_code,
        "content_length": content_length,
        "cdn_info": cdn_info,
        "cache_info": cache_info,
        "expected_region": expected_region,
        "headers_checked": check_headers,
        "performance_rating": if duration.as_millis() < 200 { "excellent" } 
                             else if duration.as_millis() < 500 { "good" }
                             else if duration.as_millis() < 1000 { "acceptable" }
                             else { "poor" },
        "status": "completed"
    }))
}
