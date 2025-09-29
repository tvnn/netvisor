use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Plex;

impl ServiceDefinition for Plex {
    fn name(&self) -> &'static str {
        "Plex Media Server"
    }
    fn description(&self) -> &'static str {
        "Media server for streaming personal content"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec![PortBase::new_tcp(32400)])
    }

    fn icon(&self) -> &'static str {
        "plex"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Plex>));
