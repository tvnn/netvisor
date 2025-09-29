use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UnifiController;

impl ServiceDefinition for UnifiController {
    fn name(&self) -> &'static str {
        "UniFi Controller"
    }
    fn description(&self) -> &'static str {
        "Ubiquiti UniFi network controller"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec![PortBase::new_tcp(2049)])
    }

    fn icon(&self) -> &'static str {
        "unifi"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<UnifiController>
));
