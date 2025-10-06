use strum_macros::EnumDiscriminants;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Eq,
    PartialEq,
    Hash,
    EnumDiscriminants,
)]
#[strum_discriminants(derive(Hash, Serialize, Deserialize))]
pub enum EntitySource {
    Manual,
    System,
    Discovery(Uuid),
    Unknown
}