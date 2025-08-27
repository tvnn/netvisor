use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{server::daemons::types::api::DaemonDiscoveryProgressResponse};

/// Discovery session status for polling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoverySessionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

// /// Discovery progress information
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DiscoveryProgress {
//     pub phase: DiscoveryPhase,
//     pub completed: usize,
//     pub total: usize,
//     pub discovered_count: usize,
//     pub progress_percent: u32,
// }

/// Complete discovery session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverySessionState {
    pub session_id: Uuid,
    pub daemon_id: Uuid,
    pub status: DiscoverySessionStatus,
    pub progress: Option<DaemonDiscoveryProgressResponse>,
    pub error_message: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}