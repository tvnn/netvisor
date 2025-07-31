use axum::{
    http::{Method, Uri},
    response::{Html, Response, IntoResponse},
    Router,
};
use clap::Parser;
use std::sync::{Arc};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod components;
mod shared;

use config::ServerConfig;
use shared::handlers::create_router;

use crate::{components::diagnostics::storage::DiagnosticStorage, shared::storage::StorageFactory};
use crate::components::tests::storage::{TestStorage};
use crate::components::nodes::storage::{NodeStorage};

pub struct AppState {
    pub config: ServerConfig,
    pub node_storage: Box<dyn NodeStorage>,
    pub test_storage: Box<dyn TestStorage>,
    pub diagnostic_storage: Box<dyn DiagnosticStorage>
    // pub discovery: Arc<Mutex<NetworkDiscovery>>,
}

#[derive(Parser)]
#[command(name = "netzoot-server")]
#[command(about = "Netzoot network diagnostics server")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "netzoot.toml")]
    config: String,
    
    /// Override server host
    #[arg(long)]
    host: Option<String>,
    
    /// Override server port
    #[arg(long)]
    port: Option<u16>,
    
    /// Override log level
    #[arg(long)]
    log_level: Option<String>,
}

async fn serve_web_assets(_uri: Uri) -> Response {
    // Temporary placeholder - will implement once UI is built
    Html("<h1>Netzoot API Server</h1><p>UI not built yet. API available at /api</p>").into_response()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let _ = dotenv::dotenv();
    
    let cli = Cli::parse();
    
    // Load configuration
    let mut config = ServerConfig::load()?;
    
    // Apply CLI overrides
    if let Some(host) = cli.host {
        config.server.host = host;
    }
    if let Some(port) = cli.port {
        config.server.port = port;
    }
    if let Some(log_level) = cli.log_level {
        config.server.log_level = log_level;
    }
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Initialize storage
    let sqlite_storage= StorageFactory::new_sqlite(&config.database_url()).await?;

    // Initialize discovery
    // let discovery = NetworkDiscovery::new()?;
    
    // Create app state
    let state = Arc::new(AppState {
        config: config.clone(),
        node_storage: sqlite_storage.nodes,
        test_storage: sqlite_storage.tests,
        diagnostic_storage: sqlite_storage.diagnostics,
        // discovery: Arc::new(Mutex::new(discovery))
    });
    
    // Create router
    let api_router = create_router().with_state(state);
    
    // Create main app with web assets fallback
    let app = Router::new()
        .merge(api_router)
        .fallback(serve_web_assets)  // Keep this line
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
    
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 Netzoot server starting on http://{}", addr);
    tracing::info!("📊 Web UI available at http://{}", addr);
    tracing::info!("🔧 API available at http://{}/api", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}