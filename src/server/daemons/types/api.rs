use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{daemon::discovery::types::base::DiscoveryPhase, server::{
    daemons::types::base::Daemon, nodes::types::base::Node, tests::types::execution::TestResult
}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonResponse {
    pub daemon: Daemon
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonListResponse {
    pub daemons: Vec<Daemon>,
    pub total: usize
}

/// Daemon registration request from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonRegistrationRequest {
    pub node: Node,
    pub daemon_id: Uuid
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
}

/// Daemon discovery response (for immediate acknowledgment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryResponse {
    pub session_id: Uuid,
}

/// Cancellation request from server to daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryCancellationRequest {
    pub session_id: Uuid,
}

/// Cancellation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryCancellationResponse {
    pub session_id: Uuid,
}

/// Progress update from daemon to server during discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonDiscoveryProgressResponse {
    pub session_id: Uuid,
    pub phase: DiscoveryPhase,
    pub completed: usize,
    pub total: usize,
    pub discovered_count: usize,
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