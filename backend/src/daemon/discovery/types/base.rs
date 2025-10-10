use std::net::IpAddr;

use crate::server::hosts::types::interfaces::Interface;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::endpoints::EndpointResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::server::{daemons::types::api::DiscoveryType, subnets::types::base::Subnet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryPhase {
    Initiated, // Initial state, set by server; all subsequent states until Finished are set by Daemon
    Started,
    Scanning,
    Complete,
    Failed,
    Cancelled,
    Finished, // Ultimate terminal state, set by server
}

#[derive(Debug, Clone)]
pub struct DiscoverySessionInfo {
    pub total_to_scan: usize,
    pub discovery_type: DiscoveryType,
    pub session_id: Uuid,
    pub daemon_id: Uuid,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct DiscoverySessionUpdate {
    pub phase: DiscoveryPhase,
    pub completed: usize,
    pub discovered_count: usize,
    pub error: Option<String>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl DiscoverySessionUpdate {
    pub fn scanning(completed: usize, discovered_count: usize) -> Self {
        Self {
            phase: DiscoveryPhase::Scanning,
            completed,
            discovered_count,
            error: None,
            finished_at: None,
        }
    }
}

impl std::fmt::Display for DiscoveryPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryPhase::Initiated => write!(f, "Session created in server"),
            DiscoveryPhase::Started => write!(f, "Session started in daemon"),
            DiscoveryPhase::Scanning => write!(f, "Scanning for active hosts"),
            DiscoveryPhase::Complete => write!(f, "Discovery complete"),
            DiscoveryPhase::Cancelled => write!(f, "Discovery cancelled"),
            DiscoveryPhase::Failed => write!(f, "Discovery failed"),
            DiscoveryPhase::Finished => write!(f, "Session finished in server"),
        }
    }
}

pub struct HostScanParams<'a> {
    pub subnet: &'a Subnet,
    pub session_id: &'a Uuid,
    pub daemon_id: &'a Uuid,
    pub started_at: &'a DateTime<Utc>,
    pub cancel: CancellationToken,
}

pub struct ProcessHostParams {
    pub host_ip: IpAddr,
    pub hostname: Option<String>,
    pub subnet: Subnet,
    pub interface: Interface,
    pub open_ports: Vec<PortBase>,
    pub endpoint_responses: Vec<EndpointResponse>,
    pub host_has_docker_client: bool,
}
