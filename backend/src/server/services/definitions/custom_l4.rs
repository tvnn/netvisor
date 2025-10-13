use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CustomLayer4;

impl ServiceDefinition for CustomLayer4 {
    fn name(&self) -> &'static str {
        "Custom (Layer 4)"
    }
    fn description(&self) -> &'static str {
        "A custom service with layer 4 bindings"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Unknown
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::None
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<CustomLayer4>
));
