use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Request from frontend to server
#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct InitiateDiscoveryRequest {
    pub daemon_id: Uuid,
}

// Response from server to frontend
#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct InitiateDiscoveryResponse {
    pub session_id: Uuid,
}
