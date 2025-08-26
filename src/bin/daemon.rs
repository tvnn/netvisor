use clap::Parser;
use netvisor::daemon::{service::DaemonClientService, storage::ConfigStore};
use std::{
    sync::Arc,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;


#[derive(Parser)]
#[command(name = "netvisor-daemon")]
#[command(about = "NetVisor network discovery and test execution daemon")]
struct Cli {
    /// Server URL to register with
    #[arg(short, long, default_value = "http://127.0.0.1:3000")]
    server_url: String,
    
    /// Daemon listen port
    #[arg(short, long, default_value = "3001")]
    port: u16,
    
    /// Daemon listen host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    
    /// Override log level
    #[arg(long, default_value = "info")]
    log_level: String,
    
    /// Skip auto-registration (for testing)
    #[arg(long)]
    no_register: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}={}", env!("CARGO_CRATE_NAME"), cli.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let storage = ConfigStore::new(default_config_path());
    let service = DaemonClientService::new(storage);

    storage.initialize().await?;

    let id = storage.get_id().await? else {
        service.register
    }

    let daemon_id = Uuid::new_v4();
    let my_ip = get_local_ip()?;
    let hostname = get_hostname();
    
    tracing::info!("🤖 NetVisor daemon starting with ID: {}", daemon_id);
    tracing::info!("🌐 Local IP: {}, Hostname: {:?}", my_ip, hostname);
    tracing::info!("🔗 Server URL: {}", cli.server_url);
    
    let state = Arc::new(DaemonState::new(
        daemon_id,
        cli.server_url.clone(),
        my_ip,
        cli.port,
        hostname,
    ));
    
    if !cli.no_register {
        tracing::info!("📝 Registering with server...");
        state.register_with_server().await?;
        
        tracing::info!("🔍 Running self-discovery...");
        state.discover_self_and_create_node().await?;
        
        let heartbeat_state = state.clone();
        tokio::spawn(async move {
            heartbeat_task(heartbeat_state).await;
        });
    } else {
        tracing::info!("⏭️  Skipping server registration");
    }
    
    let app = create_daemon_router(state);
    let addr = format!("{}:{}", cli.host, cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 NetFrog daemon listening on http://{}", addr);
    tracing::info!("🔧 Health check: http://{}/health", addr);
    tracing::info!("🔍 Discovery endpoint: http://{}/discover", addr);
    tracing::info!("🧪 Test execution endpoint: http://{}/execute_test", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}