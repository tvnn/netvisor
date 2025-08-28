use anyhow::{Context, Result};
use async_fs;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use directories_next::ProjectDirs;
use figment::{Figment, providers::{Format, Json, Env, Serialized}};
use crate::server::nodes::types::targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget};

/// CLI arguments structure (for figment integration)
#[derive(Debug)]
pub struct CliArgs {
    pub server_ip: Option<String>,
    pub server_hostname: Option<String>,
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
    pub server_hostname: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_target: Option<NodeTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_ip: None,
            server_hostname: None,
            server_port: 3000,
            port: 3001,
            name: "netvisor-daemon".to_string(),
            log_level: "info".to_string(),
            heartbeat_interval: 30,
            max_concurrent_operations: 10,
            request_timeout_ms: 30000,
            id: None,
            server_target: None,
            last_heartbeat: None,
        }
    }
}

impl AppConfig {
    pub fn load(cli_args: CliArgs) -> anyhow::Result<(Self, bool)> {
        let proj_dirs = ProjectDirs::from("com", "netvisor", "daemon")
            .ok_or_else(|| anyhow::anyhow!("Unable to determine config directory"))?;
        
        let config_path = proj_dirs.config_dir().join("daemon.json");
        let config_exists = config_path.exists();

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
        if let Some(server_hostname) = cli_args.server_hostname {
            figment = figment.merge(("server_hostname", server_hostname));
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
            .map_err(|e| anyhow::anyhow!("Configuration error: {}", e))?;

        Ok((config, config_exists))
    }

    pub fn server_target(&self) -> anyhow::Result<NodeTarget> {
        // If we have a full server_target from config file, use it
        if let Some(target) = &self.server_target {
            return Ok(target.clone());
        }

        // Otherwise, build from individual fields
        match (self.server_ip.as_ref(), self.server_hostname.as_ref()) {
            (Some(ip), _) => {
                let ip_addr = ip.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid server IP address: {}", ip))?;
                Ok(NodeTarget::IpAddress(IpAddressTargetConfig {
                    ip: ip_addr,
                    port: Some(self.server_port),
                }))
            }
            (None, Some(hostname)) => {
                Ok(NodeTarget::Hostname(HostnameTargetConfig {
                    hostname: hostname.clone(),
                    port: Some(self.server_port),
                }))
            }
            (None, None) => {
                Err(anyhow::anyhow!("Must specify either server_ip, server_hostname, or server_target"))
            }
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate server connection is specified
        if self.server_target.is_none() && self.server_ip.is_none() && self.server_hostname.is_none() {
            return Err(anyhow::anyhow!("Must specify either server_ip, server_hostname, or server_target"));
        }
        Ok(())
    }
}

/// Unified ConfigStore that uses AppConfig directly
pub struct ConfigStore {
    path: PathBuf,
    config: Arc<RwLock<AppConfig>>,
}

impl ConfigStore {
    pub fn new(path: PathBuf, server_target: NodeTarget, initial_config: AppConfig) -> Self {
        let mut config = initial_config;
        config.server_target = Some(server_target);
        
        Self {
            path,
            config: Arc::new(RwLock::new(config)),
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
        config.server_target = loaded_config.server_target.or_else(|| config.server_target.clone());
        
        tracing::info!("Loaded daemon runtime state from {}", self.path.display());
        Ok(())
    }

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

    pub async fn get_id(&self) -> Result<Option<Uuid>> {
        let config = self.config.read().await;
        Ok(config.id)
    }

    pub async fn set_id(&self, id: Uuid) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.id = Some(id);
        }
        self.save().await
    }

    pub async fn get_port(&self) -> Result<u16> {
        let config = self.config.read().await;
        Ok(config.port)
    }

    pub async fn set_port(&self, port: u16) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.port = port;
        }
        self.save().await
    }

    pub async fn set_server_endpoint(&self, endpoint: NodeTarget) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.server_target = Some(endpoint);
        }
        self.save().await
    }

    pub async fn get_server_endpoint(&self) -> Result<NodeTarget> {
        let config = self.config.read().await;
        config.server_target.clone()
            .ok_or_else(|| anyhow::anyhow!("Server target not configured"))
    }

    pub async fn get_heartbeat_interval(&self) -> Result<u64> {
        let config = self.config.read().await;
        Ok(config.heartbeat_interval)
    }

    pub async fn update_heartbeat(&self) -> Result<()> {
        {
            let mut config = self.config.write().await;
            config.last_heartbeat = Some(chrono::Utc::now());
        }
        self.save().await
    }

    pub async fn get_config(&self) -> AppConfig {
        let config = self.config.read().await;
        config.clone()
    }
}