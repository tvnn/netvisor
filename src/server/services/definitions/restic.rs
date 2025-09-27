use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Restic;

impl ServiceDefinition for Restic {
    fn name(&self) -> &'static str { "Restic" }
    fn description(&self) -> &'static str { "Fast and secure backup program" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Backup }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::Port(PortBase::new_tcp(8000))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Restic>));
