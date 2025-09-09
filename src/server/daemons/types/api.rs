use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{daemon::discovery::types::base::DiscoveryPhase, server::{
    daemons::types::base::Daemon
}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonResponse {
    pub daemon: Daemon
}

/// Daemon registration request from daemon to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonRegistrationRequest {
    pub node_id: Uuid,
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
pub struct DaemonDiscoveryUpdate {
    pub session_id: Uuid,
    pub daemon_id: Uuid,
    pub phase: DiscoveryPhase,
    pub completed: usize,
    pub total: usize,
    pub discovered_count: usize,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl DaemonDiscoveryUpdate {
    pub fn new(session_id: Uuid, daemon_id: Uuid) -> Self {
        Self {
            session_id,
            daemon_id,
            phase: DiscoveryPhase::Initiated,
            completed: 0,
            total: 0,
            discovered_count: 0,
            error: None,
            started_at: None,
            finished_at: None
        }
    }
}