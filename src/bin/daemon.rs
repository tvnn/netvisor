use clap::Parser;
use netvisor::daemon::{
    runtime::types::DaemonAppState, shared::{handlers::create_router, storage::{AppConfig, CliArgs, ConfigStore}}, utils::base::{PlatformSystemUtils, SystemUtils}
    };
use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use axum::{http::{Method}, Router};
use uuid::Uuid;
use std::{sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("🤖 NetVisor daemon starting");
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Parse CLI and convert to CliArgs
    let cli = Cli::parse();
    let cli_args = CliArgs::from(cli);
    
    // Load unified configuration
    let config = AppConfig::load(cli_args)?;
    let (_, path) = AppConfig::get_config_path()?;
    let path_str = path.to_str().unwrap_or("Config path could not be converted to string");
    
    // Initialize unified storage with full config
    let config_store = Arc::new(ConfigStore::new(path.clone(), config.clone()));
    let utils = PlatformSystemUtils::new();

    let own_addr = format!("{}:{}", utils.get_own_ip_address()?, &config_store.get_port().await?);
    let server_addr = &config_store.get_server_endpoint().await?;

    let state = DaemonAppState::new(config_store.clone(), utils).await?;    
    let runtime_service = state.services.runtime_service.clone();

    tracing::info!("🔗 Server at {}", server_addr);
    
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

    
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.port)).await?;
    
    tracing::info!("🚀 NetVisor daemon listening on http://{}", own_addr);
    tracing::info!("🔧 Health check: http://{}/health", own_addr);
    tracing::info!("🔍 Discovery endpoint: http://{}/discover", own_addr);
    tracing::info!("🧪 Test execution endpoint: http://{}/execute_test", own_addr);
    tracing::info!("📁 Config file: {:?}", path_str);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}