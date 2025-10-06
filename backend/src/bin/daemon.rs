use axum::{http::Method, Router};
use clap::Parser;
use netvisor::daemon::{
    runtime::types::DaemonAppState,
    shared::{
        handlers::create_router,
        storage::{AppConfig, CliArgs, ConfigStore},
    },
    utils::base::PlatformDaemonUtils,
};
use netvisor::server::utils::base::NetworkUtils;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "netvisor-daemon")]
#[command(about = "NetVisor network discovery and test execution daemon")]
struct Cli {
    /// Server target (IP or hostname)
    #[arg(long)]
    server_target: Option<String>,

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

    /// Daemon bind address
    #[arg(long)]
    bind_address: Option<String>,
}

impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        Self {
            server_target: cli.server_target,
            server_port: cli.server_port,
            port: cli.port,
            name: cli.name,
            bind_address: cli.bind_address,
            log_level: cli.log_level,
            heartbeat_interval: cli.heartbeat_interval,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("🤖 NetVisor daemon starting");

    // Parse CLI and convert to CliArgs
    let cli = Cli::parse();
    let cli_args = CliArgs::from(cli);

    // Load unified configuration
    let config = AppConfig::load(cli_args)?;
    let (_, path) = AppConfig::get_config_path()?;
    let path_str = path
        .to_str()
        .unwrap_or("Config path could not be converted to string");

    // Initialize unified storage with full config
    let config_store = Arc::new(ConfigStore::new(path.clone(), config.clone()));
    let utils = PlatformDaemonUtils::new();

    let daemon_id = config_store.get_id().await?;
    let own_addr = format!(
        "{}:{}",
        utils.get_own_ip_address()?,
        &config_store.get_port().await?
    );
    let server_addr = &config_store.get_server_endpoint().await?;

    let state = DaemonAppState::new(config_store, utils).await?;
    let runtime_service = state.services.runtime_service.clone();
    let discovery_service = state.services.discovery_service.clone();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🔗 Server at {}", server_addr);

    // Get or register daemon ID
    if let Some(existing_id) = runtime_service.config_store.get_host_id().await? {
        tracing::info!("📋 Existing host ID, already registered: {}", existing_id);
    } else {
        tracing::info!("📝 Registering with server...");
        // Create self as host, register with server, and save daemon ID
        discovery_service.run_self_report_discovery().await?;

        let host_id = runtime_service
            .config_store
            .get_host_id()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Host ID not set after self-report"))?;

        runtime_service
            .register_with_server(host_id, daemon_id)
            .await?;
    };

    tracing::info!("✅ Daemon ID: {}", daemon_id);

    tokio::spawn(async move {
        if let Err(e) = runtime_service.heartbeat().await {
            tracing::warn!("Failed to update heartbeat timestamp: {}", e);
        }
    });

    // Create HTTP server with config values
    let api_router = create_router().with_state(state);

    let app = Router::new().merge(api_router).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(Any),
            ),
    );

    let bind_addr = format!("{}:{}", config.bind_address, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    tracing::info!("🔍 Discovery endpoint: http://{}/discover", own_addr);
    tracing::info!("🌐 Listening on: {}", bind_addr);
    tracing::info!("📁 Config file: {:?}", path_str);

    axum::serve(listener, app).await?;

    Ok(())
}
