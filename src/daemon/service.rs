use std::{net::{IpAddr, Ipv4Addr}, sync::Arc, time::Duration};
use anyhow::Result;

use if_addrs::get_if_addrs;
use tokio::time::interval;

use crate::daemon::{storage::DaemonConfigStore, types::base::DaemonState};

pub struct DaemonClientService {
    daemon_config: Arc<DaemonConfigStore>,
}

impl DaemonClientService {
    pub fn new(daemon_config: Arc<DaemonConfigStore>,) -> Self {
        Self {
            daemon_config,
        }
    }

    
}

fn get_local_ip() -> Result<IpAddr> {
    let interfaces = get_if_addrs()?;
    
    for interface in interfaces {
        if let IpAddr::V4(ipv4) = interface.ip() {
            if !ipv4.is_loopback() && !ipv4.is_link_local() {
                return Ok(IpAddr::V4(ipv4));
            }
        }
    }
    
    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

fn get_hostname() -> Option<String> {
    hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
}

async fn heartbeat_task(state: Arc<DaemonState>) {
    let mut interval = interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        if let Err(e) = state.send_heartbeat().await {
            tracing::warn!("Heartbeat failed: {}", e);
        } else {
            tracing::debug!("Heartbeat sent successfully");
        }
    }
}