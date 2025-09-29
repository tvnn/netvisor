use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Switch;

impl ServiceDefinition for Switch {
    fn name(&self) -> &'static str {
        "Switch"
    }
    fn description(&self) -> &'static str {
        "Generic network switch for local area networking"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::NotGatewayIp,
            Pattern::AllPort(vec![PortBase::Http, PortBase::Telnet]),
        ])
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Switch>));
