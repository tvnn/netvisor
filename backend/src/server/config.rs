use anyhow::{Error, Result};
use figment::{
    providers::{Env, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

use crate::server::shared::{services::ServiceFactory, storage::seed_data::{create_internet_connectivity_host, create_public_dns_host, create_remote_host, create_remote_subnet, create_wan_subnet}};
use crate::server::shared::types::storage::StorageFactory;
use crate::server::{discovery::manager::DiscoverySessionManager, utils::base::ServerNetworkUtils};

/// CLI arguments structure (for figment integration)
#[derive(Debug)]
pub struct CliArgs {
    pub server_port: Option<u16>,
    pub log_level: Option<String>,
    pub rust_log: Option<String>,
    pub database_url: Option<String>,
    pub web_external_path: Option<String>,
}

/// Flattened server configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    // Server settings
    /// What port the server should listen on
    pub server_port: u16,

    /// Level of logs to show
    pub log_level: String,

    /// Rust log level
    pub rust_log: String,

    /// Where database should be located
    pub database_url: String,

    /// Where static web assets are located for serving
    pub web_external_path: Option<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server_port: 60072,
            log_level: "info".to_string(),
            rust_log: "".to_string(),
            database_url: "postgresql://postgres:password@localhost:5432/netvisor".to_string(),
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
        if let Some(server_port) = cli_args.server_port {
            figment = figment.merge(("server_port", server_port));
        }
        if let Some(log_level) = cli_args.log_level {
            figment = figment.merge(("log_level", log_level));
        }
        if let Some(rust_log) = cli_args.rust_log {
            figment = figment.merge(("rust_log", rust_log));
        }
        if let Some(database_url) = cli_args.database_url {
            figment = figment.merge(("database_url", database_url));
        }
        if let Some(web_external_path) = cli_args.web_external_path {
            figment = figment.merge(("web_external_path", web_external_path));
        }

        let config: ServerConfig = figment
            .extract()
            .map_err(|e| Error::msg(format!("Configuration error: {}", e)))?;

        Ok(config)
    }

    pub fn database_url(&self) -> String {
        format!("{}", self.database_url)
    }
}

pub struct AppState {
    pub config: ServerConfig,
    pub storage: StorageFactory,
    pub services: ServiceFactory,
    pub discovery_manager: DiscoverySessionManager,
    pub utils: ServerNetworkUtils,
}

impl AppState {
    pub async fn new(
        config: ServerConfig,
        discovery_manager: DiscoverySessionManager,
        utils: ServerNetworkUtils,
    ) -> Result<Arc<Self>, Error> {
        let storage = StorageFactory::new(&config.database_url()).await?;
        let services = ServiceFactory::new(&storage).await?;

        if services.host_service.get_all_hosts().await?.len() == 0 {
            tracing::info!("Seeding default data...");

            let wan_subnet = create_wan_subnet();
            let remote_subnet = create_remote_subnet();
            let (dns_host, dns_service) = create_public_dns_host(&wan_subnet);
            let (web_host, web_service) = create_internet_connectivity_host(&wan_subnet);
            let (remote_host, client_service) = create_remote_host(&remote_subnet);

            services.subnet_service.create_subnet(wan_subnet).await?;
            services.subnet_service.create_subnet(remote_subnet).await?;
            services.host_service.create_host_with_services(dns_host, vec!(dns_service)).await?;
            services.host_service.create_host_with_services(web_host, vec!(web_service)).await?;
            services.host_service.create_host_with_services(remote_host, vec!(client_service)).await?;

            tracing::info!("Default data seeded successfully");
        }

        Ok(Arc::new(Self {
            config,
            storage,
            services,
            discovery_manager,
            utils,
        }))
    }
}