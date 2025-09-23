use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Emby;

impl ServiceDefinition for Emby {
    fn name(&self) -> &'static str { "Emby" }
    fn description(&self) -> &'static str { "Personal media server with streaming capabilities" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Media }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(Port::new_tcp(8920)))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Emby>));
