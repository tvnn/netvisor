use std::net::IpAddr;

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

// Request from frontend to server
#[derive(Debug, Serialize, Deserialize)]
pub struct InitiateDiscoveryRequest {
    pub daemon_id: Uuid,
}

// Response from server to frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct InitiateDiscoveryResponse {
    pub session_id: Uuid,
}



/// Daemon discovery request from server to daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryRequest {
    pub session_id: Uuid,
}

/// Daemon discovery response (for immediate acknowledgment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryResponse {
    pub session_id: Uuid,
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
    pub session_id: Uuid,
}

/// Test result report from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonTestResult {
    pub session_id: Uuid,
    pub result: TestResult,
}