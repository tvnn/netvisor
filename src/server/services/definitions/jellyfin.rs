use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Jellyfin;

impl ServiceDefinition for Jellyfin {
    fn name(&self) -> &'static str { "Jellyfin" }
    fn description(&self) -> &'static str { "Free media server for personal streaming" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Media }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(Port::new_tcp(8096)))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Jellyfin>));
