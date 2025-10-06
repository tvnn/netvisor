use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum EntitySource {
    Manual,
    System,
    Discovery(Uuid),
    Unknown
}