// src/discovery/types.rs
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::{HashSet, HashMap};
use tokio::sync::RwLock;
use trust_dns_resolver::TokioAsyncResolver;

pub struct NetworkDiscovery {
    discovered_devices: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,
    rejected_devices: Arc<RwLock<HashSet<String>>>, // Track rejected device IPs
    progress: Arc<RwLock<DiscoveryProgress>>,
    stop_signal: Arc<RwLock<bool>>,
    resolver: TokioAsyncResolver,
}

#[derive(Deserialize)]
pub struct StartDiscoveryRequest {
    pub target_subnets: Vec<String>,
    pub discovery_depth: String,
    pub include_services: bool,
    pub snmp_communities: Vec<String>,
    pub max_concurrent: usize,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub id: String,
    pub ip: String,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub device_type: DeviceType,
    pub open_ports: Vec<u16>,
    pub services: Vec<String>,
    pub vendor: Option<String>,
    pub os_fingerprint: Option<String>,
    pub response_time_ms: u64,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub status: DiscoveryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Router,
    Switch,
    Server,
    Workstation,
    IoT,
    Printer,
    NAS,
    AccessPoint,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub target_subnets: Vec<String>,
    pub discovery_depth: DiscoveryDepth,
    pub include_services: bool,
    pub snmp_communities: Vec<String>,
    pub max_concurrent: usize,
    pub timeout_ms: u64,
    pub port_scan_enabled: bool,
    pub common_ports: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryDepth {
    Basic,     // Ping sweep only
    Standard,  // Ping + common ports + DNS
    Deep,      // Full port scan + service detection + SNMP
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryProgress {
    pub total_targets: usize,
    pub completed: usize,
    pub discovered_devices: usize,
    pub current_phase: String,
    pub is_running: bool,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            target_subnets: vec!["auto".to_string()],
            discovery_depth: DiscoveryDepth::Standard,
            include_services: true,
            snmp_communities: vec!["public".to_string(), "private".to_string()],
            max_concurrent: 50,
            timeout_ms: 3000,
            port_scan_enabled: true,
            common_ports: vec![22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 5900, 8080, 8443],
        }
    }
}