use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DhcpServer;

impl ServiceDefinition for DhcpServer {
    fn name(&self) -> &'static str { "Dhcp Server" }
    fn description(&self) -> &'static str { "A generic Dhcp server" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkCore }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::Port(Port::DHCP)
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<DhcpServer>));
