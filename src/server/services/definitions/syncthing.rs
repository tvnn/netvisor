use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Syncthing;

impl ServiceDefinition for Syncthing {
    fn name(&self) -> &'static str { "Syncthing" }
    fn description(&self) -> &'static str { "Continuous file synchronization service" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Backup }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::Port(Port::new_tcp(8384))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Syncthing>));
