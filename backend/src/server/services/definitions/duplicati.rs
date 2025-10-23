use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Duplicati;

impl ServiceDefinition for Duplicati {
    fn name(&self) -> &'static str {
        "Duplicati"
    }
    fn description(&self) -> &'static str {
        "Cross-platform backup client with encryption"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Backup
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::Http, "/", "Duplicati")
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "duplicati"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Duplicati>));
