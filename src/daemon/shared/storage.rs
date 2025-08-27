use anyhow::{Context, Result};
use async_fs;
use dirs;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::server::nodes::types::targets::NodeTarget;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DaemonConfig {
    /// Unique daemon identifier assigned by server
    pub id: Option<Uuid>,
    /// Server endpoint configuration
    pub server_endpoint: Option<NodeTarget>,
    /// Last successful heartbeat timestamp
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    /// Additional daemon configuration
    pub settings: DaemonSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSettings {
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
}

impl Default for DaemonSettings {
    fn default() -> Self {
        Self {
            heartbeat_interval: 30,
            max_concurrent_operations: 10,
            request_timeout_ms: 30000,
        }
    }
}

pub struct ConfigStore {
    path: PathBuf,
    config: Arc<RwLock<DaemonConfig>>,
}

impl ConfigStore {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            config: Arc::new(RwLock::new(DaemonConfig::default())),
        }
    }

    /// Initialize config store, creating directory and loading existing config
    pub async fn initialize(&self) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            async_fs::create_dir_all(parent).await
                .context("Failed to create config directory")?;
        }

        // Load existing config if it exists
        if self.path.exists() {
            self.load().await?;
        } else {
            tracing::info!("No existing config found, will create new config on first save");
        }

        Ok(())
    }

    /// Load configuration from disk
    async fn load(&self) -> Result<()> {
        let content = async_fs::read_to_string(&self.path).await
            .context("Failed to read config file")?;
        
        let loaded_config: DaemonConfig = serde_json::from_str(&content)
            .context("Failed to parse config file")?;
            
        let mut config = self.config.write().await;
        *config = loaded_config;
        
        tracing::info!("Loaded daemon configuration from {}", self.path.display());
        Ok(())
    }

    /// Save configuration to disk
    async fn save(&self) -> Result<()> {
        let config = self.config.read().await;
        let json = serde_json::to_string_pretty(&*config)
            .context("Failed to serialize config")?;

        // Atomic write: write to temp file then rename
        let temp_path = self.path.with_extension("tmp");
        
        async_fs::write(&temp_path, json).await
            .context("Failed to write temp config file")?;
        
        async_fs::rename(&temp_path, &self.path).await
            .context("Failed to move temp config to final location")?;

        Ok(())
    }

    /// Get daemon ID
    pub async fn get_id(&self) -> Result<Option<Uuid>> {
        let config = self.config.read().await;
        Ok(config.id)
    }

    /// Set daemon ID
    pub async fn set_id(&self, id: Uuid) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.id = Some(id);
        }
        self.save().await
    }

    /// Set server endpoint
    pub async fn set_server_endpoint(&self, endpoint: NodeTarget) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.server_endpoint = Some(endpoint);
        }
        self.save().await
    }

    /// Get server endpoint
    pub async fn get_server_endpoint(&self) -> Result<Option<NodeTarget>> {
        let config = self.config.read().await;
        Ok(config.server_endpoint.clone())
    }

    /// Update last heartbeat timestamp
    pub async fn update_heartbeat(&self) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.last_heartbeat = Some(chrono::Utc::now());
        }
        self.save().await
    }

    /// Get daemon settings
    pub async fn get_settings(&self) -> DaemonSettings {
        let config = self.config.read().await;
        config.settings.clone()
    }

    /// Update daemon settings
    pub async fn update_settings(&self, settings: DaemonSettings) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.settings = settings;
        }
        self.save().await
    }
}

/// Get default configuration path based on platform
pub fn default_config_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA")
            .unwrap_or_else(|_| "C:\\ProgramData".to_string());
        PathBuf::from(appdata).join("NetVisor").join("daemon.json")
    } else if cfg!(target_os = "macos") {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("/etc"))
            .join("netvisor")
            .join("daemon.json")
    } else {
        // Linux and other Unix-like systems
        // Try /etc/netvisor first, fallback to user config if not writable
        let system_path = PathBuf::from("/etc/netvisor/daemon.json");
        if let Some(parent) = system_path.parent() {
            if std::fs::metadata(parent).map(|m| !m.permissions().readonly()).unwrap_or(false) {
                return system_path;
            }
        }
        
        // Fallback to user config directory
        dirs::config_dir()
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".config")
            })
            .join("netvisor")
            .join("daemon.json")
    }
}