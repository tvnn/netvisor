use clap::Parser;
use netvisor::{
    daemon::{
        discovery::utils::get_local_ip_address, 
        runtime::service::DaemonRuntimeService, 
        shared::{handlers::create_router, storage::{default_config_path, ConfigStore}}
    },
    server::nodes::types::targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget},
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use hostname::get as get_hostname;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use figment::{Figment, providers::{Format, Json, Env, Serialized}};

#[derive(Parser)]
#[command(name = "netvisor-daemon")]
#[command(about = "NetVisor network discovery and test execution daemon")]
struct Cli {
    /// Server IP address
    #[arg(long)]
    server_ip: Option<String>,
    
    /// Server hostname  
    #[arg(long)]
    server_hostname: Option<String>,
    
    /// Server port
    #[arg(long)]
    server_port: Option<u16>,
    
    /// Daemon listen port
    #[arg(short, long)]
    port: Option<u16>,
    
    /// Daemon listen host
    #[arg(long)]
    host: Option<String>,

    /// Daemon name
    #[arg(long)]
    name: Option<String>,
    
    /// Log level
    #[arg(long)]
    log_level: Option<String>,

    /// Heartbeat interval in seconds
    #[arg(long)]
    heartbeat_interval: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_ip: Option<String>,
    pub server_hostname: Option<String>,
    pub server_port: u16,
    pub port: u16,
    pub host: String,
    pub name: String,
    pub log_level: String,
    pub heartbeat_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_ip: None,
            server_hostname: None,
            server_port: 3000,
            port: 3001,
            host: "127.0.0.1".to_string(),
            name: "netvisor-daemon".to_string(),
            log_level: "info".to_string(),
            heartbeat_interval: 30,
        }
    }
}

impl AppConfig {
    pub fn load() -> anyhow::Result<(Self, bool)> {
        let proj_dirs = ProjectDirs::from("com", "netvisor", "daemon")
            .ok_or_else(|| anyhow::anyhow!("Unable to determine config directory"))?;
        
        let config_path = proj_dirs.config_dir().join("daemon.json");
        let config_exists = config_path.exists();

        let cli = Cli::parse();

        // Standard configuration layering: Defaults → Config file → Env → CLI (highest priority)
        let mut figment = Figment::from(Serialized::defaults(AppConfig::default()));

        // Add config file if it exists
        if config_exists {
            figment = figment.merge(Json::file(&config_path));
        }

        // Add environment variables
        figment = figment.merge(Env::prefixed("NETVISOR_"));

        // Add CLI overrides (highest priority) - only if explicitly provided
        if let Some(server_ip) = cli.server_ip {
            figment = figment.merge(("server_ip", server_ip));
        }
        if let Some(server_hostname) = cli.server_hostname {
            figment = figment.merge(("server_hostname", server_hostname));
        }
        if let Some(server_port) = cli.server_port {
            figment = figment.merge(("server_port", server_port));
        }
        if let Some(port) = cli.port {
            figment = figment.merge(("port", port));
        }
        if let Some(host) = cli.host {
            figment = figment.merge(("host", host));
        }
        if let Some(name) = cli.name {
            figment = figment.merge(("name", name));
        }
        if let Some(log_level) = cli.log_level {
            figment = figment.merge(("log_level", log_level));
        }
        if let Some(heartbeat_interval) = cli.heartbeat_interval {
            figment = figment.merge(("heartbeat_interval", heartbeat_interval));
        }

        let config: AppConfig = figment.extract()
            .map_err(|e| anyhow::anyhow!("Configuration error: {}", e))?;

        Ok((config, config_exists))
    }

    pub fn server_target(&self) -> anyhow::Result<NodeTarget> {
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
                Err(anyhow::anyhow!("Must specify either server_ip or server_hostname"))
            }
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate server connection is specified
        if self.server_ip.is_none() && self.server_hostname.is_none() {
            return Err(anyhow::anyhow!("Must specify either server_ip or server_hostname"));
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration with automatic layering
    let (config, from_file) = AppConfig::load()?;
    
    // Validate configuration
    config.validate()?;

    // Initialize logging with config value
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}={}", env!("CARGO_CRATE_NAME"), config.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if from_file {
        tracing::info!("🤖 NetVisor daemon starting (config file + overrides)");
    } else {
        tracing::info!("🤖 NetVisor daemon starting (CLI/environment only)");
    }

    // Get server target from config
    let server_target = config.server_target()?;
    tracing::info!("🔗 Server target: {}", server_target);
    
    // Initialize storage with server target
    let storage = Arc::new(ConfigStore::new(default_config_path(), server_target.clone()));
    storage.initialize().await?;
    
    let mut runtime_service = DaemonRuntimeService::new(storage.clone());
    
    // Set daemon port from configuration
    runtime_service.config_store.set_port(config.port).await?;
    
    // Get or register daemon ID
    let daemon_id = if let Some(existing_id) = runtime_service.config_store.get_id().await? {
        tracing::info!("📋 Using existing daemon ID: {}", existing_id);
        existing_id
    } else {
        // Get local network info
        let local_ip = get_local_ip_address()?;
        let hostname = get_hostname()
            .ok()
            .map(|os_str| os_str.to_string_lossy().into_owned());

        tracing::info!("🌐 Local IP: {}, Hostname: {:?}", local_ip, hostname);
        tracing::info!("📝 Registering with server...");
        
        // Create self as node, register with server, and save returned daemon ID
        let node = runtime_service.create_self_as_node().await?;
        let new_id = runtime_service.register_with_server(node).await?;
        runtime_service.config_store.set_id(new_id).await?;
        
        new_id
    };
    
    tracing::info!("✅ Daemon ID: {}", daemon_id);
        
    tokio::spawn(async move {
        if let Err(e) = runtime_service.heartbeat().await {
            tracing::warn!("Failed to update heartbeat timestamp: {}", e);
        }
    });
    
    // Create HTTP server with config values
    let app = create_router().with_state(storage);
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 NetVisor daemon listening on http://{}", addr);
    tracing::info!("🔧 Health check: http://{}/health", addr);
    tracing::info!("🔍 Discovery endpoint: http://{}/discover", addr);
    tracing::info!("🧪 Test execution endpoint: http://{}/execute_test", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}