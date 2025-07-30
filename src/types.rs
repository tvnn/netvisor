use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: String,
    pub name: String,
    pub target: String,
    pub node_type: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NetworkNode {
    pub fn new(name: String, target: String, node_type: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            target,
            node_type,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConfig {
    pub target: Option<String>,
    pub timeout: Option<u64>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub expected_response: Option<String>,
    pub follow_redirects: Option<bool>,
    pub user_agent: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub method: Option<String>,
    pub body: Option<String>,
    pub verify_ssl: Option<bool>,
    pub max_redirects: Option<u32>,
    pub custom_headers: Option<HashMap<String, String>>,
    
    // DNS specific
    pub dns_server: Option<String>,
    pub record_type: Option<String>,
    
    // Network scanning
    pub max_hosts: Option<u32>,
    pub concurrent_requests: Option<u32>,
    pub packet_count: Option<u32>,
    pub packet_size: Option<u32>,
    
    // Performance testing
    pub duration: Option<u64>,
    pub max_hops: Option<u32>,
    pub timeout_per_hop: Option<u64>,
    pub resolve_hostnames: Option<bool>,
    
    // Email specific
    pub email: Option<String>,
    pub use_tls: Option<bool>,
    pub use_starttls: Option<bool>,
    
    // Additional fields for extensibility
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl Default for CheckConfig {
    fn default() -> Self {
        Self {
            target: None,
            timeout: Some(5000),
            port: None,
            username: None,
            password: None,
            expected_response: None,
            follow_redirects: Some(true),
            user_agent: Some("Netzoot/1.0".to_string()),
            headers: None,
            method: Some("GET".to_string()),
            body: None,
            verify_ssl: Some(true),
            max_redirects: Some(5),
            custom_headers: None,
            dns_server: None,
            record_type: Some("A".to_string()),
            max_hosts: Some(254),
            concurrent_requests: Some(50),
            packet_count: Some(4),
            packet_size: Some(64),
            duration: Some(10000),
            max_hops: Some(30),
            timeout_per_hop: Some(5000),
            resolve_hostnames: Some(true),
            email: None,
            use_tls: Some(true),
            use_starttls: Some(true),
            extra: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Check {
    pub id: String,
    pub check_type: String,
    pub config: CheckConfig,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub description: String,
    pub checks: Vec<Check>,
    pub failure_actions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub layers: Vec<Layer>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Test {
    pub fn new(name: String, description: String, version: String, layers: Vec<Layer>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            version,
            layers,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub check_type: String,
    pub config: CheckConfig,
    pub success: bool,
    pub message: String,
    pub error: Option<String>,
    pub details: Option<serde_json::Value>,
    pub duration: u64,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub checks: Vec<CheckResult>,
    pub success: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub duration: u64,
    pub failure_actions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResults {
    pub id: String,
    pub test_id: String,
    pub test_name: String,
    pub timestamp: DateTime<Utc>,
    pub layers: Vec<LayerResult>,
    pub success: bool,
    pub total_duration: u64,
}

impl DiagnosticResults {
    pub fn new(test_id: String, test_name: String, layers: Vec<LayerResult>) -> Self {
        let success = layers.iter().all(|layer| layer.success);
        let total_duration = layers.iter().map(|layer| layer.duration).sum();
        
        Self {
            id: Uuid::new_v4().to_string(),
            test_id,
            test_name,
            timestamp: Utc::now(),
            layers,
            success,
            total_duration,
        }
    }
}