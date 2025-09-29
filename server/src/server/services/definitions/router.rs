use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Router;

impl ServiceDefinition for Router {
    fn name(&self) -> &'static str {
        "Router"
    }
    fn description(&self) -> &'static str {
        "Network router providing routing and gateway services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::IsGatewayIp,
            Pattern::AnyPort(vec![
                PortBase::Ssh,
                PortBase::Http,
                PortBase::Https,
                PortBase::Dhcp,
            ]),
        ])
    }

    fn is_gateway(&self) -> bool {
        true
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Router>));
