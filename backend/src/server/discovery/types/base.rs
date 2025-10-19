use crate::server::services::types::patterns::MatchConfidence;
use crate::server::services::types::patterns::MatchDetails;
use crate::server::services::types::patterns::MatchReason;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumDiscriminants;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, Serialize, Deserialize))]
#[serde(tag = "type", content = "metadata")]
pub enum EntitySource {
    Manual,
    System,
    Discovery(MatchMetadata),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct MatchMetadata {
    #[serde(flatten)]
    pub discovery_type: DiscoveryType,
    pub daemon_id: Uuid,
    pub details: Option<MatchDetails>,
}

impl MatchMetadata {
    pub fn new_certain(discovery_type: DiscoveryType, daemon_id: Uuid, reason: &str) -> Self {
        Self {
            discovery_type,
            daemon_id,
            details: Some(MatchDetails {
                reason: MatchReason::Reason(reason.to_string()),
                confidence: MatchConfidence::Certain,
            }),
        }
    }

    pub fn new_no_details(discovery_type: DiscoveryType, daemon_id: Uuid) -> Self {
        Self {
            discovery_type,
            daemon_id,
            details: None,
        }
    }
}

impl Default for MatchMetadata {
    fn default() -> Self {
        Self {
            discovery_type: DiscoveryType::Network,
            daemon_id: Uuid::new_v4(),
            details: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Display, Copy)]
#[serde(tag = "discovery_type")]
pub enum DiscoveryType {
    SelfReport,
    Network,
    Docker { host_id: Uuid },
    Proxmox { host_id: Uuid },
}
