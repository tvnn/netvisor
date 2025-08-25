use serde::{Deserialize, Serialize};
use crate::components::nodes::capabilities::base::NodeCapability;
use crate::components::nodes::types::types::NodeType;

use crate::components::{
    nodes::types::{targets::NodeTarget}
};
use crate::components::nodes::capabilities::base::{deserialize_capabilities_from_discriminants};
use crate::shared::types::metadata::TypeMetadata;

#[derive(Serialize, Debug, Clone)]
pub struct TestConfigSchema {
    pub test_info: TypeMetadata,
    pub contextual_description: String,
    pub compatibility: CompatibilityStatus,
    pub requirements_met: bool,
    pub compatibility_reason: Option<String>,
    pub fields: Vec<ConfigField>,
    pub warnings: Vec<ValidationMessage>,
    pub errors: Vec<ValidationMessage>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum CompatibilityStatus {
    Compatible,
    Incompatible,
    Conditional,
}

#[derive(Serialize, Debug, Clone)]
pub struct ConfigField {
    pub id: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub help_text: Option<String>,
    pub placeholder: Option<String>,
    pub advanced: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct FieldType {
    pub base_type: String,
    pub constraints: serde_json::Value,
    pub options: Option<Vec<SelectOption>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub disabled: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ValidationMessage {
    pub message: String,
    pub field_id: Option<String>,
    pub severity: MessageSeverity,
}

#[derive(Serialize, Debug, Clone)]
pub enum MessageSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeContext {
    pub node_id: Option<String>,
    pub node_type: NodeType,
    #[serde(
        // serialize_with = "serialize_capabilities_as_discriminants",
        deserialize_with = "deserialize_capabilities_from_discriminants"
    )]
    pub capabilities: Vec<NodeCapability>,
    pub target: NodeTarget,
}