use clap::Parser;
use netvisor::daemon::{
        discovery::service::DaemonDiscoveryService, runtime::{service::DaemonRuntimeService, types::base::DaemonState}, shared::{handlers::create_router, storage::{AppConfig, CliArgs, ConfigStore}}
    };
use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use axum::{http::{Method}, Router};
use uuid::Uuid;
use std::{sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use directories_next::ProjectDirs;

#[derive(Parser)]
#[command(name = "netvisor-daemon")]
#[command(about = "NetVisor network discovery and test execution daemon")]
struct Cli {
    /// Server IP address
    #[arg(long)]
    server_ip: String,
        
    /// Server port
    #[arg(long)]
    server_port: u16,
    
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

impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        Self {
            server_ip: Some(cli.server_ip),
            server_port: Some(cli.server_port),
            port: cli.port,
            name: cli.name,
            log_level: cli.log_level,
            heartbeat_interval: cli.heartbeat_interval,
        }
    }
}

fn get_runtime_config_path() -> anyhow::Result<std::path::PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "netvisor", "daemon")
        .ok_or_else(|| anyhow::anyhow!("Unable to determine config directory"))?;
    
    Ok(proj_dirs.data_dir().join("runtime.json"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI and convert to CliArgs
    let cli = Cli::parse();
    let cli_args = CliArgs::from(cli);
    
    // Load unified configuration
    let (config, from_file) = AppConfig::load(cli_args)?;
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    if from_file {
        tracing::info!("🤖 NetVisor daemon starting (config file + overrides)");
    } else {
        tracing::info!("🤖 NetVisor daemon starting (CLI/environment only)");
    }
    
    // Initialize unified storage with full config
    // Use separate path for runtime state to avoid conflicts
    let runtime_path = get_runtime_config_path()?;
    let storage = Arc::new(ConfigStore::new(runtime_path, config.clone()));
    storage.initialize().await?;
    
    let mut runtime_service = DaemonRuntimeService::new(storage.clone());
    let discovery_service = Arc::new(DaemonDiscoveryService::new(storage.clone()));
    
    // Get or register daemon ID
    let daemon_id = if let Some(existing_id) = runtime_service.config_store.get_id().await? {
        tracing::info!("📋 Using existing daemon ID: {}", existing_id);
        existing_id
    } else {        
        tracing::info!("📝 Registering with server...");
        
        let daemon_id = Uuid::new_v4();

        // Create self as node, register with server, and save daemon ID
        let node = runtime_service.create_self_as_node(daemon_id).await?;
        tracing::info!("🌐 Local IP: {}, Hostname: {:?}", node.base.target.to_string(), node.base.hostname);

        runtime_service.register_with_server(node, daemon_id).await?;
        runtime_service.config_store.set_id(daemon_id).await?;
        
        daemon_id
    };
    
    tracing::info!("✅ Daemon ID: {}", daemon_id);
        
    tokio::spawn(async move {
        if let Err(e) = runtime_service.heartbeat().await {
            tracing::warn!("Failed to update heartbeat timestamp: {}", e);
        }
    });

    let state = Arc::new(DaemonState {
        config: storage,
        discovery_service
    });
    
    // Create HTTP server with config values
    let api_router = create_router().with_state(state);

    let app = Router::new()
        .merge(api_router)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any),
                ),
        );

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 NetVisor daemon listening on http://{}", addr);
    tracing::info!("🔧 Health check: http://{}/health", addr);
    tracing::info!("🔍 Discovery endpoint: http://{}/discover", addr);
    tracing::info!("🧪 Test execution endpoint: http://{}/execute_test", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}