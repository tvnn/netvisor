use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use crate::server::daemons::{storage::DaemonStorage, types::base::Daemon};

pub struct DaemonService {
    daemon_storage: Arc<dyn DaemonStorage>,
}

impl DaemonService {
    pub fn new(daemon_storage: Arc<dyn DaemonStorage>,) -> Self {
        Self {
            daemon_storage,
        }
    }

    pub async fn register_daemon(&self, daemon: Daemon) -> Result<Daemon> {
        self.daemon_storage.create(&daemon).await?;
        Ok(daemon)
    }

    pub async fn get_daemon(&self, id: &Uuid) -> Result<Option<Daemon>> {
        self.daemon_storage.get_by_id(id).await
    }

    pub async fn receive_heartbeat(&self, mut daemon: Daemon) -> Result<Daemon> {
        let now = chrono::Utc::now();

        daemon.last_seen = now;

        self.daemon_storage.update(&daemon).await?;

        Ok(daemon)
    }
}