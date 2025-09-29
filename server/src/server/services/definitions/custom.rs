use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Custom;

impl ServiceDefinition for Custom {
    fn name(&self) -> &'static str {
        "Custom"
    }
    fn description(&self) -> &'static str {
        "A custom service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Unknown
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::None
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Custom>));
