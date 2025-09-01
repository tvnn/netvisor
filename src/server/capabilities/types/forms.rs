use serde::{Serialize};

use crate::server::{shared::{forms::types::fields::*, types::metadata::TypeMetadata}, tests::types::base::TestDiscriminants};

#[derive(Serialize, Debug, Clone)]
pub struct CapabilityConfigForm {
    pub capability_info: TypeMetadata,
    pub capability_fields: Vec<ConfigField>,
    pub test_sections: Vec<TestSection>,
    pub warnings: Vec<ValidationMessage>,
    pub errors: Vec<ValidationMessage>,
    pub system_assigned: bool
}

#[derive(Serialize, Debug, Clone)]
pub struct TestSection {
    pub test_type: TestDiscriminants,  // Uses your existing enum
    pub test_info: TypeMetadata,
    pub test_fields: Vec<ConfigField>,
}