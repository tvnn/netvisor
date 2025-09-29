use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::definitions::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DhcpServer;

impl ServiceDefinition for DhcpServer {
    fn name(&self) -> &'static str {
        "Dhcp Server"
    }
    fn description(&self) -> &'static str {
        "A generic Dhcp server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::Port(PortBase::Dhcp)
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<DhcpServer>));
