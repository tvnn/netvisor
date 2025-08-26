use std::{path::{Path, PathBuf}};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Context;
use crate::server::nodes::types::{targets::NodeTarget};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DaemonConfig {
    pub id: Option<Uuid>,
    pub server_endpoint: Option<NodeTarget>,
    #[serde(serialize_with = "chrono::serde::ts_seconds_option::serialize")]
    pub last_heartbeat: Option<DateTime<Utc>>
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            id: None,
            server_endpoint: None,
            last_heartbeat: None,
        }
    }
}


pub struct DaemonConfigStore {
    path: PathBuf,
    config: DaemonConfig,
}

impl DaemonConfigStore {
    /// Create a new config store with the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            config: DaemonConfig::default(),
        }
    }

    /// Initialize config store - loads existing config or creates default
    pub async fn initialize(&mut self) -> Result<()> {
        if self.path.exists() {
            self.load().await?;
        } else {
            // Create parent directories if they don't exist
            if let Some(parent) = self.path.parent() {
                async_fs::create_dir_all(parent).await
                    .context("Failed to create config directory")?;
            }
            self.save().await?;
        }
        Ok(())
    }

    /// Load configuration from file
    pub async fn load(&mut self) -> Result<()> {
        let content = async_fs::read_to_string(&self.path).await
            .context("Failed to read config file")?;
        
        self.config = serde_json::from_str(&content)
            .context("Failed to parse config JSON")?;
        
        Ok(())
    }

    /// Save configuration to file atomically
    pub async fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.config)
            .context("Failed to serialize config")?;

        // Atomic write: write to temp file then rename
        let temp_path = self.path.with_extension("tmp");
        
        async_fs::write(&temp_path, json).await
            .context("Failed to write temp config file")?;
        
        async_fs::rename(&temp_path, &self.path).await
            .context("Failed to move temp config to final location")?;

        Ok(())
    }

    pub async fn get_id(&self) -> Option<Uuid> {
        self.config.id
    }

    pub async fn set_id(&mut self, id: Uuid) -> Result<()> {
        self.config.id = Some(id);
        self.save().await
    }

    /// Set server endpoint
    pub async fn set_server_endpoint(&mut self, endpoint: NodeTarget) -> Result<()> {
        self.config.server_endpoint = Some(endpoint);
        self.save().await
    }

    /// Update last heartbeat timestamp
    pub async fn update_heartbeat(&mut self) -> Result<()> {
        self.config.last_heartbeat = Some(
            chrono::Utc::now()
        );
        self.save().await
    }
}

pub fn default_config_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA")
            .unwrap_or_else(|_| "C:\\ProgramData".to_string());
        PathBuf::from(appdata).join("NetVisor").join("daemon.json")
    } else {
        PathBuf::from("/etc/netvisor/daemon.json")
            .parent()
            .and_then(|p| {
                // Fallback to user config if /etc is not writable
                if p.metadata().map(|m| m.permissions().readonly()).unwrap_or(true) {
                    dirs::config_dir().map(|d| d.join("netvisor").join("daemon.json"))
                } else {
                    Some(PathBuf::from("/etc/netvisor/daemon.json"))
                }
            })
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".netvisor")
                    .join("daemon.json")
            })
    }
}