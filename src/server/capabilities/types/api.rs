use serde::{Deserialize};
use crate::server::{capabilities::types::{base::CapabilityDiscriminants}, nodes::types::base::NodeContext};

#[derive(Deserialize)]
pub struct CapabilityFormRequest {
    pub capability_types: Option<Vec<CapabilityDiscriminants>>,  // None = get forms for all capabilities
    pub node_context: NodeContext,
}