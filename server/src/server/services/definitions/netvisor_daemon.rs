use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::{PortBase};
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NetvisorDaemon;

impl ServiceDefinition for NetvisorDaemon {
    fn name(&self) -> &'static str { "NetVisor Daemon" }
    fn description(&self) -> &'static str { "NetVisor Daemon for network scanning" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Netvisor }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::Port(PortBase::new_tcp(60073))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NetvisorDaemon>));
