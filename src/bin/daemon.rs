use clap::Parser;
use netvisor::{
    daemon::runtime::{
        service::DaemonClientService, 
        storage::{ConfigStore, default_config_path}, 
        handlers::create_daemon_router
    },
    server::nodes::types::targets::{NodeTarget, IpAddressTargetConfig, HostnameTargetConfig},
};
use std::{sync::Arc, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use if_addrs::get_if_addrs;
use hostname::get as get_hostname;

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
    #[arg(long, default_value = "3000")]
    server_port: u16,
    
    /// Daemon listen port
    #[arg(short, long, default_value = "3001")]
    port: u16,
    
    /// Daemon listen host
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Daemon name
    #[arg(long, default_value = "netvisor-daemon")]
    name: Option<String>,
    
    /// Override log level
    #[arg(long, default_value = "info")]
    log_level: String,
    
    /// Skip auto-registration (for testing)
    #[arg(long)]
    no_register: bool,

    /// Heartbeat interval in seconds
    #[arg(long, default_value = "30")]
    heartbeat_interval: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}={}", env!("CARGO_CRATE_NAME"), cli.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Determine server target from CLI args
    let server_target = match (cli.server_ip.as_ref(), cli.server_hostname.as_ref()) {
        (Some(ip), _) => {
            let ip_addr = ip.parse()
                .map_err(|_| anyhow::anyhow!("Invalid server IP address: {}", ip))?;
            NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: ip_addr,
                port: Some(cli.server_port),
            })
        }
        (None, Some(hostname)) => {
            NodeTarget::Hostname(HostnameTargetConfig {
                hostname: hostname.clone(),
                port: Some(cli.server_port),
            })
        }
        (None, None) => {
            tracing::error!("❌ Must specify either --server-ip or --server-hostname");
            std::process::exit(1);
        }
    };

    tracing::info!("🤖 NetVisor daemon starting");
    tracing::info!("🔗 Server target: {}", server_target);
    
    // Initialize storage and service
    let storage = Arc::new(ConfigStore::new(default_config_path()));
    storage.initialize().await?;
    
    let mut service = DaemonClientService::new(storage);
    
    // Save server target to config
    service.config_store.set_server_endpoint(server_target.clone()).await?;
    
    // Get or register daemon ID
    let daemon_id = if let Some(existing_id) = service.config_store.get_id().await? {
        tracing::info!("📋 Using existing daemon ID: {}", existing_id);
        existing_id
    } else if !cli.no_register {
        // Get local network info
        let local_ip = get_local_ip()?;
        let hostname = get_hostname()
            .ok()
            .map(|os_str| os_str.to_string_lossy().into_owned());

        let name = cli.name.unwrap_or("netvisor-daemon".to_string());
        
        tracing::info!("🌐 Local IP: {}, Hostname: {:?}", local_ip, hostname);
        tracing::info!("📝 Registering with server...");
        
        // Register with server and save ID
        let new_id = service.register_with_server(&server_target, name, local_ip, cli.port, hostname).await?;
        service.config_store.set_id(new_id).await?;
        
        new_id
    } else {
        tracing::error!("❌ No daemon ID found and registration skipped");
        std::process::exit(1);
    };
    
    tracing::info!("✅ Daemon ID: {}", daemon_id);
    
    // Start heartbeat task if not skipping registration
    if !cli.no_register {
        let heartbeat_service = Arc::new(service);
        let heartbeat_target = server_target.clone();
        let heartbeat_interval = Duration::from_secs(cli.heartbeat_interval);
        
        // Clone for heartbeat task
        let heartbeat_service_clone = heartbeat_service.clone();
        tokio::spawn(async move {
            heartbeat_task(heartbeat_service_clone, heartbeat_target, daemon_id, heartbeat_interval).await;
        });
        
        // Create HTTP server
        let app = create_daemon_router().with_state(heartbeat_service);
        let addr = format!("{}:{}", cli.host, cli.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        tracing::info!("🚀 NetVisor daemon listening on http://{}", addr);
        tracing::info!("🔧 Health check: http://{}/health", addr);
        tracing::info!("🔍 Discovery endpoint: http://{}/discover", addr);
        tracing::info!("🧪 Test execution endpoint: http://{}/execute_test", addr);
        
        axum::serve(listener, app).await?;
    } else {
        tracing::info!("⏭️  Skipping heartbeat task and HTTP server");
    }
    
    Ok(())
}

/// Get the primary local IP address
fn get_local_ip() -> anyhow::Result<std::net::IpAddr> {
    let interfaces = get_if_addrs()?;
    
    // Try to find a non-loopback, non-link-local IPv4 address
    for interface in &interfaces {
        let ip = interface.addr.ip();
        if !ip.is_loopback() && !ip.is_multicast() {
            if let std::net::IpAddr::V4(ipv4) = ip {
                if !ipv4.is_link_local() {
                    return Ok(ip);
                }
            }
        }
    }
    
    // Fallback to any non-loopback address
    for interface in &interfaces {
        let ip = interface.addr.ip();
        if !ip.is_loopback() {
            return Ok(ip);
        }
    }
    
    // Last resort fallback
    Ok(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
}

/// Background heartbeat task
async fn heartbeat_task(
    service: Arc<DaemonClientService>, 
    server_target: NodeTarget, 
    daemon_id: uuid::Uuid,
    interval: Duration,
) {
    let mut interval_timer = tokio::time::interval(interval);
    interval_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    
    loop {
        interval_timer.tick().await;
        
        match service.send_heartbeat(&server_target, &daemon_id).await {
            Ok(()) => {
                // Update last heartbeat timestamp in config
                if let Err(e) = service.config_store.update_heartbeat().await {
                    tracing::warn!("Failed to update heartbeat timestamp: {}", e);
                }
                tracing::trace!("💓 Heartbeat sent successfully");
            }
            Err(e) => {
                tracing::warn!("❤️‍🩹 Heartbeat failed: {}", e);
                // Continue trying - don't exit on heartbeat failures
            }
        }
    }
}