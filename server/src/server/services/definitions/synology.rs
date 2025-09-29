use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Synology;

impl ServiceDefinition for Synology {
    fn name(&self) -> &'static str {
        "Synology DSM"
    }
    fn description(&self) -> &'static str {
        "Synology DiskStation Manager NAS system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "Synology")
    }

    fn icon(&self) -> &'static str {
        "synology"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Synology>));
