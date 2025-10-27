use axum::{Router, http::Method};
use clap::Parser;
use netvisor::daemon::{
    discovery::service::{base::Discovery, self_report::SelfReportDiscovery},
    runtime::types::DaemonAppState,
    shared::{
        handlers::create_router,
        storage::{AppConfig, CliArgs, ConfigStore},
    },
    utils::base::{DaemonUtils, PlatformDaemonUtils},
};
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
    daemon_port: Option<u16>,

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

    /// Concurrent scans for discovery
    #[arg(long)]
    concurrent_scans: Option<usize>,
}

impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        Self {
            server_target: cli.server_target,
            server_port: cli.server_port,
            daemon_port: cli.daemon_port,
            name: cli.name,
            bind_address: cli.bind_address,
            log_level: cli.log_level,
            heartbeat_interval: cli.heartbeat_interval,
            concurrent_scans: cli.concurrent_scans,
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
    let has_docker_client = utils.get_own_docker_socket().await?;
    let server_addr = &config_store.get_server_endpoint().await?;

    let state = DaemonAppState::new(config_store, utils).await?;
    let runtime_service = state.services.runtime_service.clone();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(format!(
            "netvisor={},daemon={}",
            config.log_level, config.log_level
        )))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Server at {}", server_addr);

    // Get or register daemon ID
    if let Some(existing_id) = runtime_service.config_store.get_host_id().await? {
        tracing::info!("Existing host ID, already registered: {}", existing_id);
    } else {
        tracing::info!("Registering with server...");

        let network_id = match runtime_service.config_store.get_network_id().await? {
            Some(network_id) => network_id,
            None => {
                tracing::info!("No network ID provided, getting default network ID from server...");
                let network_id = runtime_service.get_default_network().await?.id;
                let _ = runtime_service
                    .config_store
                    .set_network_id(network_id)
                    .await;
                network_id
            }
        };

        // Create self as host, register with server, and save daemon ID
        let discovery = Discovery::new(
            state.services.discovery_service.clone(),
            state.services.discovery_manager.clone(),
            SelfReportDiscovery::default(),
        );
        discovery.run_self_report_discovery().await?;

        let host_id = runtime_service
            .config_store
            .get_host_id()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Host ID not set after self-report"))?;

        runtime_service
            .register_with_server(host_id, daemon_id, network_id)
            .await?;

        if has_docker_client {
            discovery.run_self_report_docker_discovery().await?;
        }
    };

    tracing::info!("Daemon ID: {}", daemon_id);

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

    let bind_addr = format!("{}:{}", config.bind_address, config.daemon_port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    tracing::info!("🌐 Listening on: {}", bind_addr);
    tracing::info!("📁 Config file: {:?}", path_str);

    axum::serve(listener, app).await?;

    Ok(())
}
