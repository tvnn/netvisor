use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumDiscriminants;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants, Copy)]
#[strum_discriminants(derive(Hash, Serialize, Deserialize))]
pub enum EntitySource {
    Manual,
    System,
    Discovery(DiscoveryType, Uuid),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Display, Copy)]
pub enum DiscoveryType {
    SelfReport,
    Network,
    Docker { host_id: Uuid },
}
