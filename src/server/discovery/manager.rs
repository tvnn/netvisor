use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{Utc};

use crate::{server::{
    daemons::{
        types::{api::{
            DaemonDiscoveryProgressResponse,
        }},
    }, discovery::types::session::{DiscoverySessionState, DiscoverySessionStatus}
}};

/// Server-side session management for discovery
pub struct DiscoverySessionManager {
    sessions: RwLock<HashMap<Uuid, DiscoverySessionState>>,
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

        let session_state = DiscoverySessionState {
            session_id,
            daemon_id,
            status: DiscoverySessionStatus::Running,
            progress: None,
            error_message: None,
            started_at: Utc::now(),
            completed_at: None,
        };

        self.sessions.write().await.insert(session_id, session_state);
        self.daemon_sessions.write().await.insert(daemon_id, session_id);

        tracing::info!("Created discovery session {} for daemon {}", session_id, daemon_id);
        Ok(())
    }

    /// Get session state
    pub async fn get_session(&self, session_id: &Uuid) -> Option<DiscoverySessionState> {
        self.sessions.read().await.get(session_id).cloned()
    }

    /// Update progress for a session
    pub async fn update_progress(&self, progress: DaemonDiscoveryProgressResponse) -> Result<(), anyhow::Error> {
        if let Some(session) = self.sessions.write().await.get_mut(&progress.session_id) {
            session.progress = Some(progress);
            tracing::debug!("Updated progress for session {}: {} ({}/{})", 
                           session.session_id, session.progress.as_ref().unwrap().phase,
                           session.progress.as_ref().unwrap().completed,
                           session.progress.as_ref().unwrap().total);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    /// Mark session as completed
    pub async fn complete_session(&self, session_id: &Uuid) -> Result<(), anyhow::Error> {
        if let Some(session) = self.sessions.write().await.get_mut(session_id) {
            session.status = DiscoverySessionStatus::Completed;
            session.completed_at = Some(Utc::now());
            
            // Remove from daemon sessions mapping
            self.daemon_sessions.write().await.remove(&session.daemon_id);
            
            tracing::info!("Completed discovery session {}", session_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    /// Mark session as failed
    pub async fn fail_session(&self, session_id: &Uuid, error_message: String) -> Result<(), anyhow::Error> {
        if let Some(session) = self.sessions.write().await.get_mut(session_id) {
            session.status = DiscoverySessionStatus::Failed;
            session.error_message = Some(error_message);
            session.completed_at = Some(Utc::now());
            
            // Remove from daemon sessions mapping
            self.daemon_sessions.write().await.remove(&session.daemon_id);
            
            tracing::warn!("Failed discovery session {}: {}", session_id, session.error_message.as_ref().unwrap());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    /// Cancel session
    pub async fn cancel_session(&self, session_id: &Uuid) -> Option<Uuid> {
        if let Some(session) = self.sessions.write().await.get_mut(session_id) {
            let daemon_id = session.daemon_id;
            session.status = DiscoverySessionStatus::Cancelled;
            session.completed_at = Some(Utc::now());
            
            // Remove from daemon sessions mapping
            self.daemon_sessions.write().await.remove(&daemon_id);
            
            tracing::info!("Cancelled discovery session {} for daemon {}", session_id, daemon_id);
            Some(daemon_id)
        } else {
            None
        }
    }

    /// Check if daemon is discovering
    pub async fn is_daemon_discovering(&self, daemon_id: &Uuid) -> Option<Uuid> {
        self.daemon_sessions.read().await.get(daemon_id).copied()
    }

    /// Cleanup old completed sessions (call periodically)
    pub async fn cleanup_old_sessions(&self, max_age_hours: i64) {
        let cutoff = Utc::now() - chrono::Duration::hours(max_age_hours);
        let mut sessions = self.sessions.write().await;
        let mut daemon_sessions = self.daemon_sessions.write().await;
        
        let mut to_remove = Vec::new();
        for (session_id, session) in sessions.iter() {
            if let Some(completed_at) = session.completed_at {
                if completed_at < cutoff {
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
    pub async fn get_active_session_count(&self) -> usize {
        self.daemon_sessions.read().await.len()
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