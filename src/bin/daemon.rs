use clap::Parser;
use netvisor::{
    daemon::{
        discovery::utils::get_local_ip_address, 
        runtime::service::DaemonRuntimeService, 
        shared::{handlers::create_router, storage::{ConfigStore, AppConfig, CliArgs}}
    },
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use hostname::get as get_hostname;
use directories_next::ProjectDirs;

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

impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        Self {
            server_ip: cli.server_ip,
            server_hostname: cli.server_hostname,
            server_port: cli.server_port,
            port: cli.port,
            host: cli.host,
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
    
    // Initialize unified storage with full config
    // Use separate path for runtime state to avoid conflicts
    let runtime_path = get_runtime_config_path()?;
    let storage = Arc::new(ConfigStore::new(runtime_path, server_target.clone(), config.clone()));
    storage.initialize().await?;
    
    let mut runtime_service = DaemonRuntimeService::new(storage.clone());
    
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