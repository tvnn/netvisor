use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, Serialize, Deserialize))]
pub enum EntitySource {
    Manual,
    System,
    Discovery(Uuid),
    Unknown,
}
