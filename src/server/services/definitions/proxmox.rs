use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Proxmox;

impl ServiceDefinition for Proxmox {
    fn name(&self) -> &'static str { "Proxmox VE" }
    fn description(&self) -> &'static str { "Open-source virtualization management platform" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Virtualization }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(PortBase::new_tcp(8006)))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Proxmox>));
