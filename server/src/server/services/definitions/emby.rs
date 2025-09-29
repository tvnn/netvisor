use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Emby;

impl ServiceDefinition for Emby {
    fn name(&self) -> &'static str { "Emby" }
    fn description(&self) -> &'static str { "Personal media server with streaming capabilities" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Media }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(PortBase::new_tcp(8920)))
    }

    fn icon(&self) -> &'static str {
        "emby"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Emby>));
