use std::sync::Arc;

use anyhow::Error;
use tokio::sync::RwLock;

pub struct DaemonDiscoverySessionManager {
    current_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl DaemonDiscoverySessionManager {
    pub fn new() -> Self {
        Self {
            current_task: Arc::new(RwLock::new(None)),
        }
    }

    /// Check if discovery is currently running
    pub async fn is_discovery_running(&self) -> bool {
        let task_guard = self.current_task.read().await;
        if let Some(handle) = task_guard.as_ref() {
            !handle.is_finished()
        } else {
            false
        }
    }

    /// Set the current discovery task for cancellation
    pub async fn set_current_task(&self, handle: tokio::task::JoinHandle<()>) -> Result<(), Error> {
        if self.is_discovery_running().await {
            return Err(Error::msg("Discovery already running on this daemon"));
        }
        
        *self.current_task.write().await = Some(handle);
        Ok(())
    }

    /// Cancel current discovery task
    pub async fn cancel_current_discovery(&self) -> bool {
        let mut task_guard = self.current_task.write().await;
        if let Some(handle) = task_guard.take() {
            handle.abort();
            true
        } else {
            false
        }
    }

    /// Clear completed task
    pub async fn clear_completed_task(&self) {
        let mut task_guard = self.current_task.write().await;
        if let Some(handle) = task_guard.as_ref() {
            if handle.is_finished() {
                *task_guard = None;
            }
        }
    }
}