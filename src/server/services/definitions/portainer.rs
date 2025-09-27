use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Portainer;

impl ServiceDefinition for Portainer {
    fn name(&self) -> &'static str { "Portainer" }
    fn description(&self) -> &'static str { "Container management web interface" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Virtualization }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec![PortBase::new_tcp(9000), PortBase::new_tcp(9443)])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Portainer>));
