use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Jellyfin;

impl ServiceDefinition for Jellyfin {
    fn name(&self) -> &'static str {
        "Jellyfin"
    }
    fn description(&self) -> &'static str {
        "Free media server for personal streaming"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(8096))
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "jellyfin"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Jellyfin>));
