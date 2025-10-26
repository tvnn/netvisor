use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::services::types::categories::ServiceCategory;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TopologyRequestOptions {
    pub network_ids: Vec<Uuid>,
    pub group_docker_bridges_by_host: bool,
    pub infra_service_categories: Vec<ServiceCategory>,
    pub hide_service_categories: Vec<ServiceCategory>,
    pub show_gateway_as_infra_service: bool,
    pub show_interface_edges: bool,
    pub edge_type: String,
}
