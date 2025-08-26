use std::net::IpAddr;

use cidr::IpCidr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::{
    daemons::types::base::Daemon, nodes::types::base::Node, tests::types::execution::TestResult
};

/// Daemon registration request from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonRegistrationRequest {
    pub ip: IpAddr,
    pub port: u16,
    pub name: String,
    pub hostname: Option<String>,
}

/// Daemon registration response from server to daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonRegistrationResponse {
    pub daemon: Daemon
}

/// Daemon discovery request from server to daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryRequest {
    pub session_id: Uuid,
    pub target_subnets: Vec<IpCidr>,
    pub discovery_depth: String,
    pub include_services: bool,
    pub snmp_communities: Vec<String>,
    pub max_concurrent: usize,
    pub timeout_ms: u64,
}

/// Daemon discovery response (for immediate acknowledgment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryResponse {
    pub success: bool,
    pub session_id: Uuid,
    pub message: String,
}

/// Progress update from daemon to server during discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryProgress {
    pub session_id: Uuid,
    pub phase: String,
    pub completed: usize,
    pub total: usize,
    pub discovered_count: usize,
}

/// Discovered node report from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonNodeReport {
    pub session_id: Uuid,
    pub node: Node,
}

/// Test execution request from server to daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonTestRequest {
    pub session_id: Uuid,
    pub node: Node,
}

/// Test execution response from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonTestResponse {
    pub success: bool,
    pub session_id: Uuid,
    pub message: String,
}

/// Test result report from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonTestResult {
    pub session_id: Uuid,
    pub result: TestResult,
}