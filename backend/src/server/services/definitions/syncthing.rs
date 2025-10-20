use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Syncthing;

impl ServiceDefinition for Syncthing {
    fn name(&self) -> &'static str {
        "Syncthing"
    }
    fn description(&self) -> &'static str {
        "Continuous file synchronization service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Backup
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(8384))
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "syncthing"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Syncthing>));
