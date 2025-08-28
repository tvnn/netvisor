use std::collections::HashMap;
use anyhow::Error;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{Utc};

use crate::{daemon::discovery::types::base::DiscoveryPhase, server::daemons::types::api::DaemonDiscoveryUpdate};

/// Server-side session management for discovery
pub struct DiscoverySessionManager {
    sessions: RwLock<HashMap<Uuid, DaemonDiscoveryUpdate>>, // session_id -> session state mapping
    daemon_sessions: RwLock<HashMap<Uuid, Uuid>>, // daemon_id -> session_id mapping
}

impl DiscoverySessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            daemon_sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new discovery session
    pub async fn create_session(&self, session_id: Uuid, daemon_id: Uuid) -> Result<(), anyhow::Error> {
        // Check if daemon is already running discovery
        if self.daemon_sessions.read().await.contains_key(&daemon_id) {
            return Err(anyhow::anyhow!("Daemon is already running discovery"));
        }

        let session_state = DaemonDiscoveryUpdate::new(session_id, daemon_id);

        self.sessions.write().await.insert(session_id, session_state);
        self.daemon_sessions.write().await.insert(daemon_id, session_id);

        tracing::info!("Created discovery session {} for daemon {}", session_id, daemon_id);
        Ok(())
    }

    /// Get session state
    pub async fn get_session(&self, session_id: &Uuid) -> Option<DaemonDiscoveryUpdate> {
        self.sessions.read().await.get(session_id).cloned()
    }

    /// Update progress for a session
    pub async fn update_session(&self, update: DaemonDiscoveryUpdate) -> Result<Uuid, Error> {
        if let Some(session) = self.sessions.write().await.get_mut(&update.session_id) {
            let daemon_id = session.daemon_id;
            tracing::debug!("Updated session {}: {} ({}/{})", 
                           update.session_id, update.phase,
                           update.completed,
                           update.total);
            *session = update;

            if matches!(session.phase, DiscoveryPhase::Cancelled | DiscoveryPhase::Complete | DiscoveryPhase::Finished) {
                // Remove from daemon sessions mapping
                match &session.error {
                    Some(e) => tracing::error!("{} discovery session {} with error {}", &session.phase, &session.session_id, e),
                    None => tracing::info!("{} discovery session {}", &session.phase, &session.session_id)
                }
                self.daemon_sessions.write().await.remove(&session.daemon_id);
                session.finished_at = Some(Utc::now());
            }

            Ok(daemon_id)
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    pub async fn terminate_session(&self, session_id: &Uuid, phase: DiscoveryPhase) -> Result<Uuid, Error> {
        if let Some(session) = self.sessions.write().await.get(session_id) {
            let mut update = session.clone();
            update.phase = phase;
            self.update_session(update).await
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    /// Check if daemon is discovering
    pub async fn is_daemon_discovering(&self, daemon_id: &Uuid) -> Option<Uuid> {
        self.daemon_sessions.read().await.get(daemon_id).copied()
    }

    /// Check for timed out sessions and mark them as failed
    pub async fn check_timeouts(&self, timeout_minutes: i64) {
        let timeout_cutoff = Utc::now() - chrono::Duration::minutes(timeout_minutes);
        let mut sessions = self.sessions.write().await;        
        let mut timed_out_sessions = Vec::new();
        
        for (session_id, session) in sessions.iter_mut() {
            if session.finished_at == None && session.started_at < Some(timeout_cutoff) {
                session.error = Some("Discovery timed out".to_string());
                match self.terminate_session(session_id, DiscoveryPhase::Failed).await {
                    Ok(_) => (),
                    Err(e) => tracing::error!("Could not terminate session: {}", e)
                };
                timed_out_sessions.push((*session_id, session.daemon_id));
                tracing::warn!("Discovery session {} timed out after {} minutes", session_id, timeout_minutes);
            }
        }
    }

    /// Cleanup old completed sessions (call periodically)
    pub async fn cleanup_old_sessions(&self, max_age_hours: i64) {
        let cutoff = Utc::now() - chrono::Duration::hours(max_age_hours);
        let mut sessions = self.sessions.write().await;
        let mut daemon_sessions = self.daemon_sessions.write().await;
        
        let mut to_remove = Vec::new();
        for (session_id, session) in sessions.iter() {
            if let Some(finished_at) = session.finished_at {
                if finished_at < cutoff {
                    to_remove.push(*session_id);
                }
            }
        }
        
        for session_id in to_remove {
            if let Some(session) = sessions.remove(&session_id) {
                daemon_sessions.remove(&session.daemon_id);
                tracing::debug!("Cleaned up old discovery session {}", session_id);
            }
        }
    }

    /// Get active session count for monitoring
    pub async fn get_active_sessions(&self) -> Vec<DaemonDiscoveryUpdate> {
        self.sessions.read().await
            .values()
            .filter(|session| !matches!(
                session.phase, 
                DiscoveryPhase::Cancelled | DiscoveryPhase::Complete | DiscoveryPhase::Finished
            ))
            .cloned()
            .collect()
    }
}

// Extended DaemonService methods for discovery coordination



    // /// Handle discovery completion
    // pub async fn complete_discovery_session(
    //     &self,
    //     session_id: Uuid,
    //     discovery_manager: &ServerDiscoverySessionManager
    // ) -> Result<(), anyhow::Error> {
    //     discovery_manager.complete_session(&session_id).await?;
    //     Ok(())
    // }

    // /// Handle discovery failure
    // pub async fn fail_discovery_session(
    //     &self,
    //     session_id: Uuid,
    //     error_message: String,
    //     discovery_manager: &ServerDiscoverySessionManager
    // ) -> Result<(), anyhow::Error> {
    //     discovery_manager.fail_session(&session_id, error_message).await?;
    //     Ok(())
    // }

// Note: Add ServerDiscoverySessionManager to your AppState:
/*
pub struct AppState {
    // ... existing fields
    pub discovery_manager: ServerDiscoverySessionManager,
    pub daemon_storage: Arc<dyn DaemonStorage>,
}

// Optional: Add periodic cleanup task
pub async fn start_discovery_cleanup_task(discovery_manager: Arc<ServerDiscoverySessionManager>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_hours(1));
    loop {
        interval.tick().await;
        discovery_manager.cleanup_old_sessions(24).await; // Keep sessions for 24 hours
    }
}
*/