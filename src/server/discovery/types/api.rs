use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::discovery::types::session::DiscoverySessionState;


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

/// Response for status polling endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryStatusResponse {
    pub session: DiscoverySessionState,
}