use axum::{
    http::{header, Method, StatusCode, Uri},
    response::{Html, Response},
    Router,
};
use clap::Parser;
use rust_embed::RustEmbed;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handlers;
mod storage;
mod types;
mod network_checks;

use config::ServerConfig;
use handlers::create_router;
use storage::{SqliteStorage, Storage};

#[derive(RustEmbed)]
#[folder = "../netzoot-ui/build"]
struct WebAssets;

#[derive(Debug)]
pub struct AppState {
    pub config: ServerConfig,
    pub storage: Box<dyn Storage>,
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

async fn serve_web_assets(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    
    // If it's an API route, return 404
    if path.starts_with("api/") {
        return StatusCode::NOT_FOUND.into_response();
    }
    
    // Try to serve the requested file
    let file_path = if path.is_empty() || path == "index.html" {
        "index.html"
    } else {
        path
    };
    
    match WebAssets::get(file_path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(content.data.into())
                .unwrap()
        }
        None => {
            // For SPA, return index.html for unknown routes
            match WebAssets::get("index.html") {
                Some(content) => Html(String::from_utf8_lossy(&content.data).to_string()).into_response(),
                None => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    let storage = SqliteStorage::new(&config.database_url()).await?;
    
    // Create app state
    let state = Arc::new(AppState {
        config: config.clone(),
        storage: Box::new(storage),
    });
    
    // Create router
    let api_router = create_router().with_state(state);
    
    // Create main app with web assets fallback
    let app = Router::new()
        .merge(api_router)
        .fallback(serve_web_assets)
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