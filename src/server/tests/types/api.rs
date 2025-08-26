use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::server::tests::types::{base::TestDiscriminants, schema::{NodeContext, TestConfigSchema}};

#[derive(Deserialize)]
pub struct SchemaRequest {
    pub test_types: Option<Vec<TestDiscriminants>>,  // None = get all schemas
    pub node_context: NodeContext,
}

#[derive(Serialize)]
pub struct SchemaResponse {
    pub schemas: HashMap<TestDiscriminants, TestConfigSchema>,
}