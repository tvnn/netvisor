use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::server::{capabilities::types::{base::CapabilityDiscriminants, forms::CapabilityConfigForm}, nodes::types::base::NodeContext};

#[derive(Deserialize)]
pub struct FormRequest {
    pub capability_types: Option<Vec<CapabilityDiscriminants>>,  // None = get forms for all capabilities
    pub node_context: NodeContext,
}

#[derive(Serialize)]
pub struct FormResponse {
    pub forms: HashMap<CapabilityDiscriminants, CapabilityConfigForm>,
}