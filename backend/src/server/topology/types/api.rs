use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default)]
pub struct TopologyRequestOptions {
    pub network_id: Uuid,
    pub group_docker_bridges_by_host: bool,
}
