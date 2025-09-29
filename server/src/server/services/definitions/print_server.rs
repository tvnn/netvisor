use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PrintServer;

impl ServiceDefinition for PrintServer {
    fn name(&self) -> &'static str { "Print Server" }
    fn description(&self) -> &'static str { "A generic printing service" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Printer }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(PortBase::Ipp, PortBase::LdpTcp, PortBase::LdpUdp))
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PrintServer>));
