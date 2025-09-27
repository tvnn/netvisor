use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use figment::{Figment, providers::{Env, Serialized}};

use crate::server::discovery::manager::DiscoverySessionManager;
use crate::server::shared::services::ServiceFactory;
use crate::server::shared::types::storage::StorageFactory;

/// CLI arguments structure (for figment integration)
#[derive(Debug)]
pub struct CliArgs {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub log_level: Option<String>,
    pub database_path: Option<String>,
    pub web_external_path: Option<String>,
}

/// Flattened server configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    // Server settings
    pub host: String,
    pub port: u16,
    pub log_level: String,
    
    // Database settings
    pub database_path: PathBuf,
    
    // Web settings
    pub web_external_path: Option<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 60072,
            log_level: "info".to_string(),
            database_path: PathBuf::from("./netvisor.db"),
            web_external_path: None,
        }
    }
}

impl ServerConfig {
    pub fn load(cli_args: CliArgs) -> anyhow::Result<Self> {
        // Standard configuration layering: Defaults → Env → CLI (highest priority)
        let mut figment = Figment::from(Serialized::defaults(ServerConfig::default()));

        // Add environment variables with NETVISOR_ prefix
        figment = figment.merge(Env::prefixed("NETVISOR_"));

        // Add CLI overrides (highest priority) - only if explicitly provided
        if let Some(host) = cli_args.host {
            figment = figment.merge(("host", host));
        }
        if let Some(port) = cli_args.port {
            figment = figment.merge(("port", port));
        }
        if let Some(log_level) = cli_args.log_level {
            figment = figment.merge(("log_level", log_level));
        }
        if let Some(database_path) = cli_args.database_path {
            figment = figment.merge(("database_path", database_path));
        }
        if let Some(web_external_path) = cli_args.web_external_path {
            figment = figment.merge(("web_external_path", web_external_path));
        }

        let config: ServerConfig = figment.extract()
            .map_err(|e| Error::msg(format!("Configuration error: {}", e)))?;

        // Ensure database directory exists
        if let Some(parent) = config.database_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if !config.database_path.exists() {
            std::fs::File::create(&config.database_path)?;
        }

        Ok(config)
    }

    pub fn database_url(&self) -> String {
        format!("sqlite:{}", self.database_path.display())
    }
}

pub struct AppState {
    pub config: ServerConfig,
    pub storage: StorageFactory,
    pub services: ServiceFactory,
    pub discovery_manager: DiscoverySessionManager
}

impl AppState {
    pub async fn new(config: ServerConfig, discovery_manager: DiscoverySessionManager) -> Result<Arc<Self>, Error> {
        let storage = StorageFactory::new_sqlite(&config.database_url()).await?;
        let services = ServiceFactory::new(&storage).await?;
        
        Ok(Arc::new(Self { config, storage, services, discovery_manager }))
    }
}