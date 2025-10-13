use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default)]
pub struct TopologyRequestOptions {
    pub group_docker_bridges_by_host: bool,
}
