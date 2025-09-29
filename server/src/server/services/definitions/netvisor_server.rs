use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NetvisorServer;

impl ServiceDefinition for NetvisorServer {
    fn name(&self) -> &'static str { "NetVisor Server" }
    fn description(&self) -> &'static str { "NetVisor server for network management" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Netvisor }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(PortBase::new_tcp(60072)))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NetvisorServer>));
