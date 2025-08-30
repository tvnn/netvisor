use serde::{Serialize};

use crate::server::{shared::{forms::types::fields::*, types::metadata::TypeMetadata}, tests::types::base::TestDiscriminants};

#[derive(Serialize, Debug, Clone)]
pub struct CapabilityConfigForm {
    pub capability_info: TypeMetadata,
    
    // Capability-specific configuration fields (port, path, etc.)
    pub capability_fields: Vec<ConfigField>,
    
    // Auto-assigned test sections with their own fields
    pub test_sections: Vec<TestSection>,
    
    pub warnings: Vec<ValidationMessage>,
    pub errors: Vec<ValidationMessage>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TestSection {
    pub test_type: TestDiscriminants,  // Uses your existing enum
    pub test_info: TypeMetadata,
    pub test_fields: Vec<ConfigField>,
}