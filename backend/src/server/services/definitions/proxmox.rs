use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Proxmox;

impl ServiceDefinition for Proxmox {
    fn name(&self) -> &'static str {
        "Proxmox VE"
    }
    fn description(&self) -> &'static str {
        "Open-source virtualization management platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Virtualization
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(8006))
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "proxmox"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Proxmox>));
