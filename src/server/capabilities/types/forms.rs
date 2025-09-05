use serde::{Serialize};

use crate::server::{shared::{forms::types::fields::*, types::metadata::TypeMetadata}};

#[derive(Serialize, Debug, Clone)]
pub struct CapabilityConfigForm {
    pub capability_info: TypeMetadata,
    pub capability_fields: Vec<ConfigField>,
    pub warnings: Vec<ValidationMessage>,
    pub errors: Vec<ValidationMessage>,
    pub system_assigned: bool
}