use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub layers: Vec<Layer>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Test {
    pub fn new(name: String, description: Option<String>, layers: Vec<Layer>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            layers,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub description: String,
    pub checks: Vec<Check>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Check {
    pub r#type: String,
    pub config: CheckConfig,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTestRequest {
    pub name: String,
    pub description: Option<String>,
    pub layers: serde_json::Value, // Will be parsed as Vec<Layer>
}

#[derive(Deserialize)]
pub struct ExecuteCheckRequest {
    pub config: CheckConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConfig {
    // Basic fields
    pub target: Option<String>,
    pub port: Option<i64>,
    pub protocol: Option<String>, // 'http' | 'https'
    pub timeout: Option<u64>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub attempts: Option<u32>,
    
    // DNS fields
    pub test_domain: Option<String>,
    pub service_type: Option<String>, // 'google' | 'cloudflare' | 'pihole' | 'auto'
    
    // HTTP/Service health fields
    pub expected_status: Option<u16>,
    pub max_response_time: Option<u64>,
    
    // Email server fields
    pub use_tls: Option<bool>,
    pub use_ssl: Option<bool>,
    
    // SSL certificate fields
    pub min_days_until_expiry: Option<u32>,
    pub check_chain: Option<bool>,
    
    // Local network fields
    pub interface: Option<String>,
    pub subnet: Option<String>,
    pub concurrent_scans: Option<u32>,
    
    // Protocol-specific fields
    pub passive_mode: Option<bool>,
    pub check_banner: Option<bool>,
    pub db_type: Option<String>,
    
    // Performance test fields
    pub test_duration: Option<u32>,
    pub test_type: Option<String>, // 'download' | 'upload'
    pub packet_count: Option<u32>,
    pub interval_ms: Option<u64>,
    pub sample_count: Option<u32>,
    
    // Advanced test fields
    pub start_size: Option<u32>,
    pub max_size: Option<u32>,
    pub max_hops: Option<u32>,
    pub timeout_per_hop: Option<u64>,
    pub resolve_hostnames: Option<bool>,
    pub port_range: Option<String>,
    pub scan_type: Option<String>, // 'tcp' | 'udp'
    
    // CDN fields
    pub expected_region: Option<String>,
    pub check_headers: Option<bool>,
    
    // Additional protocol fields
    pub max_time_drift: Option<u64>,
    pub bind_dn: Option<String>,
    pub transport: Option<String>, // 'udp' | 'tcp'
}

// impl Default for CheckConfig {
//     fn default() -> Self {
//         Self {
//             target: None,
//             timeout: Some(5000),
//             port: None,
//             username: None,
//             password: None,
//             expected_response: None,
//             follow_redirects: Some(true),
//             user_agent: Some("Netzoot/1.0".to_string()),
//             headers: None,
//             method: Some("GET".to_string()),
//             body: None,
//             verify_ssl: Some(true),
//             max_redirects: Some(5),
//             custom_headers: None,
//             dns_server: None,
//             record_type: Some("A".to_string()),
//             max_hosts: Some(254),
//             concurrent_requests: Some(50),
//             packet_count: Some(4),
//             packet_size: Some(64),
//             duration: Some(10000),
//             max_hops: Some(30),
//             timeout_per_hop: Some(5000),
//             resolve_hostnames: Some(true),
//             email: None,
//             use_tls: Some(true),
//             use_starttls: Some(true),
//         }
//     }
// }