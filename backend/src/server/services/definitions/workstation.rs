use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Workstation;

impl ServiceDefinition for Workstation {
    fn name(&self) -> &'static str {
        "Workstation"
    }
    fn description(&self) -> &'static str {
        "Desktop computer for productivity work"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Workstation
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortBase::Rdp),
            Pattern::Port(PortBase::Samba),
        ])
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Workstation>));
