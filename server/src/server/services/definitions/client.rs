use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Client;

impl ServiceDefinition for Client {
    fn name(&self) -> &'static str {
        "Client"
    }
    fn description(&self) -> &'static str {
        "A generic client device that initiates connections to services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Mobile
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::None // Clients aren't typically discoverable
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Client>));
