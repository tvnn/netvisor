use anyhow::{Context, Error, Result};
use async_fs;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use directories_next::ProjectDirs;
use figment::{Figment, providers::{Format, Json, Env, Serialized}};

/// CLI arguments structure (for figment integration)
#[derive(Debug)]
pub struct CliArgs {
    pub server_ip: Option<String>,
    pub server_port: Option<u16>,
    pub port: Option<u16>,
    pub name: Option<String>,
    pub log_level: Option<String>,
    pub heartbeat_interval: Option<u64>,
}

/// Unified configuration struct that handles both startup and runtime config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    // Server connection (CLI/startup config)
    pub server_ip: Option<String>,
    pub server_port: u16,
    
    // Daemon settings (CLI/startup config)
    pub port: u16,
    pub name: String,
    pub log_level: String,
    
    // Runtime settings (persisted)
    pub heartbeat_interval: u64,
    pub max_concurrent_operations: usize,
    pub request_timeout_ms: u64,
    
    // Runtime state (persisted)
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<Uuid>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_ip: None,
            server_port: 60072,
            port: 60073,
            name: "netvisor-daemon".to_string(),
            log_level: "info".to_string(),
            heartbeat_interval: 30,
            max_concurrent_operations: 10,
            request_timeout_ms: 30000,
            id: Uuid::new_v4(),
            last_heartbeat: None,
            node_id: None
        }
    }
}

impl AppConfig {
    pub fn get_config_path() -> Result<(bool, PathBuf)> {
        let proj_dirs = ProjectDirs::from("com", "netvisor", "daemon")
            .ok_or_else(|| anyhow::anyhow!("Unable to determine config directory"))?;
        
        let config_path = proj_dirs.config_dir().join("config.json");
        Ok((config_path.exists(), config_path))
    }
    pub fn load(cli_args: CliArgs) -> anyhow::Result<Self> {        
        let (config_exists, config_path) = AppConfig::get_config_path()?;

        // Standard configuration layering: Defaults → Config file → Env → CLI (highest priority)
        let mut figment = Figment::from(Serialized::defaults(AppConfig::default()));

        // Add config file if it exists
        if config_exists {
            figment = figment.merge(Json::file(&config_path));
        }

        // Add environment variables
        figment = figment.merge(Env::prefixed("NETVISOR_"));

        // Add CLI overrides (highest priority) - only if explicitly provided
        if let Some(server_ip) = cli_args.server_ip {
            figment = figment.merge(("server_ip", server_ip));
        }
        if let Some(server_port) = cli_args.server_port {
            figment = figment.merge(("server_port", server_port));
        }
        if let Some(port) = cli_args.port {
            figment = figment.merge(("port", port));
        }
        if let Some(name) = cli_args.name {
            figment = figment.merge(("name", name));
        }
        if let Some(log_level) = cli_args.log_level {
            figment = figment.merge(("log_level", log_level));
        }
        if let Some(heartbeat_interval) = cli_args.heartbeat_interval {
            figment = figment.merge(("heartbeat_interval", heartbeat_interval));
        }

        let config: AppConfig = figment.extract()
            .map_err(|e| Error::msg(format!("Configuration error: {}", e)))?;

        Ok(config)
    }
}

pub struct ConfigStore {
    path: PathBuf,
    config: Arc<RwLock<AppConfig>>,
}

impl ConfigStore {
    pub fn new(path: PathBuf, initial_config: AppConfig) -> Self {        
        Self {
            path,
            config: Arc::new(RwLock::new(initial_config)),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            async_fs::create_dir_all(parent).await
                .context("Failed to create config directory")?;
        }

        // Load existing config if it exists and merge with current config
        if self.path.exists() {
            self.load().await?;
        } else {
            tracing::info!("No existing runtime config found, will create new on first save");
        }

        Ok(())
    }

    async fn load(&self) -> Result<()> {
        let content = async_fs::read_to_string(&self.path).await
            .context("Failed to read config file")?;
        
        let loaded_config: AppConfig = serde_json::from_str(&content)
            .context("Failed to parse config file")?;
            
        // Merge loaded runtime state with current config
        let mut config = self.config.write().await;
        config.id = loaded_config.id;
        config.last_heartbeat = loaded_config.last_heartbeat;
        
        tracing::info!("Loaded daemon runtime state from {}", self.path.display());
        Ok(())
    }

    async fn save(&self, config: &AppConfig) -> Result<()> {
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

    pub async fn get_id(&self) -> Result<Uuid> {
        let config = self.config.read().await;
        Ok(config.id)
    }

    pub async fn set_id(&self, id: Uuid) -> Result<()> {
        let mut config = self.config.write().await;
        config.id = id;
        self.save(&config.clone()).await
    }

    pub async fn get_node_id(&self) -> Result<Option<Uuid>> {
        let config = self.config.read().await;
        Ok(config.node_id)
    }

    pub async fn set_node_id(&self,  node_id: Uuid) -> Result<()> {
        let mut config = self.config.write().await;
        config.node_id = Some(node_id);
        self.save(&config.clone()).await
    }

    pub async fn get_port(&self) -> Result<u16> {
        let config = self.config.read().await;
        Ok(config.port)
    }

    pub async fn set_port(&self, port: u16) -> Result<()> {
        let mut config = self.config.write().await;
        config.port = port;
        self.save(&config.clone()).await
    }

    pub async fn set_server_ip(&self, ip: String) -> Result<()> {
        let mut config = self.config.write().await;
        config.server_ip = Some(ip);
        self.save(&config.clone()).await
    }

    pub async fn get_server_endpoint(&self) -> Result<String> {
        let config = self.config.read().await;

        if let Some(ip) = &config.server_ip {
            Ok(format!("http://{}:{}", ip, config.server_port))
        } else {
            Err(Error::msg("No IP configured for server"))
        }
    }

    pub async fn get_heartbeat_interval(&self) -> Result<u64> {
        let config = self.config.read().await;
        Ok(config.heartbeat_interval)
    }

    pub async fn update_heartbeat(&self) -> Result<()> {
        let mut config = self.config.write().await;
        config.last_heartbeat = Some(chrono::Utc::now());
        self.save(&config.clone()).await
    }

    pub async fn get_config(&self) -> AppConfig {
        let config = self.config.read().await;
        config.clone()
    }
}