use axum::{http::Method, Router};
use clap::Parser;
use netvisor::server::{
    config::{AppState, CliArgs, ServerConfig},
    discovery::manager::DiscoverySessionManager,
    shared::handlers::create_router,
    utils::base::{NetworkUtils, ServerNetworkUtils},
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "netvisor-server")]
#[command(about = "NetVisor server")]
struct Cli {
    /// Override server port
    #[arg(long)]
    port: Option<u16>,

    /// Override log level
    #[arg(long)]
    log_level: Option<String>,

    /// Override database path
    #[arg(long)]
    database_path: Option<String>,

    /// Override web external path
    #[arg(long)]
    web_external_path: Option<String>,
}

impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        Self {
            port: cli.port,
            log_level: cli.log_level,
            database_path: cli.database_path,
            web_external_path: cli.web_external_path,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    let cli = Cli::parse();
    let cli_args = CliArgs::from(cli);

    // Load configuration using figment
    let config = ServerConfig::load(cli_args)?;
    let listen_addr = format!("0.0.0.0:{}", &config.port);

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create app state
    let state = AppState::new(
        config,
        DiscoverySessionManager::new(),
        ServerNetworkUtils::new(),
    )
    .await?;

    // Create discovery cleanup task
    let cleanup_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
        loop {
            interval.tick().await;

            // Check for timeouts (fail sessions running > 10 minutes)
            // cleanup_state.discovery_manager.check_timeouts(10).await;

            // Clean up old sessions (remove completed sessions > 24 hours old)
            cleanup_state
                .discovery_manager
                .cleanup_old_sessions(24)
                .await;
        }
    });

    // Create router
    let api_router = if let Some(static_path) = &state.config.web_external_path {
        Router::new()
            .nest_service("/", ServeDir::new(static_path))
            .merge(create_router())
            .with_state(state)
    } else {
        tracing::warn!("Could not load web assets due to no web_external_path");
        create_router().with_state(state)
    };

    // Create main app
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

    let listener = tokio::net::TcpListener::bind(&listen_addr).await?;
    let actual_port = listener.local_addr()?.port();

    tracing::info!("🚀 NetVisor server started successfully");
    tracing::info!("📊 Web UI: http://<your-ip>:{}", actual_port);
    tracing::info!("🔧 API: http://<your-ip>:{}/api", actual_port);

    axum::serve(listener, app).await?;

    Ok(())
}
