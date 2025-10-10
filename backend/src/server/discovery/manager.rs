use anyhow::Error;
use chrono::Utc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    daemon::discovery::types::base::DiscoveryPhase,
    server::daemons::types::api::DiscoveryUpdatePayload,
};

/// Server-side session management for discovery
pub struct DiscoverySessionManager {
    sessions: RwLock<HashMap<Uuid, DiscoveryUpdatePayload>>, // session_id -> session state mapping
    daemon_sessions: RwLock<HashMap<Uuid, Uuid>>,            // daemon_id -> session_id mapping
}

impl DiscoverySessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            daemon_sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new discovery session
    pub async fn create_session(
        &self,
        session_id: Uuid,
        daemon_id: Uuid,
    ) -> Result<DiscoveryUpdatePayload, anyhow::Error> {
        // Check if daemon is already running discovery
        if self.daemon_sessions.read().await.contains_key(&daemon_id) {
            return Err(anyhow::anyhow!("Daemon is already running discovery"));
        }

        let session_state = DiscoveryUpdatePayload::new(session_id, daemon_id);

        self.sessions
            .write()
            .await
            .insert(session_id, session_state.clone());
        self.daemon_sessions
            .write()
            .await
            .insert(daemon_id, session_id);

        tracing::info!(
            "Created discovery session {} for daemon {}",
            session_id,
            daemon_id
        );
        Ok(session_state)
    }

    /// Get session state
    pub async fn get_session(&self, session_id: &Uuid) -> Option<DiscoveryUpdatePayload> {
        self.sessions.read().await.get(session_id).cloned()
    }

    /// Update progress for a session
    pub async fn update_session(&self, update: DiscoveryUpdatePayload) -> Result<Uuid, Error> {
        if let Some(session) = self.sessions.write().await.get_mut(&update.session_id) {
            let daemon_id = session.daemon_id;
            tracing::debug!(
                "Updated session {}: {} ({}/{})",
                update.session_id,
                update.phase,
                update.completed,
                update.total
            );
            *session = update;

            if matches!(
                session.phase,
                DiscoveryPhase::Cancelled | DiscoveryPhase::Complete | DiscoveryPhase::Finished
            ) {
                // Remove from daemon sessions mapping
                match &session.error {
                    Some(e) => tracing::error!(
                        "{} discovery session {} with error {}",
                        &session.phase,
                        &session.session_id,
                        e
                    ),
                    None => tracing::info!(
                        "{} discovery session {}",
                        &session.phase,
                        &session.session_id
                    ),
                }
                self.daemon_sessions
                    .write()
                    .await
                    .remove(&session.daemon_id);
                session.finished_at = Some(Utc::now());
            }

            Ok(daemon_id)
        } else {
            Err(anyhow::anyhow!("Session not found"))
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
    pub async fn get_active_sessions(&self) -> Vec<DiscoveryUpdatePayload> {
        self.sessions
            .read()
            .await
            .values()
            .filter(|session| {
                !matches!(
                    session.phase,
                    DiscoveryPhase::Cancelled | DiscoveryPhase::Complete | DiscoveryPhase::Finished
                )
            })
            .cloned()
            .collect()
    }
}

impl Default for DiscoverySessionManager {
    fn default() -> Self {
        Self::new()
    }
}
