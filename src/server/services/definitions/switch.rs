use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Switch;

impl ServiceDefinition for Switch {
    fn name(&self) -> &'static str { "Switch" }
    fn description(&self) -> &'static str { "Generic network switch for local area networking" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkCore }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::NotGatewayIp,
            Pattern::AnyOf(vec![
                // Managed switch with SNMP
                Pattern::AllPort(vec![Port::SNMP, Port::HTTP]),
                // SSH-managed switch
                Pattern::AllPort(vec![Port::SSH, Port::HTTP]),
                // Basic web-managed switch
                Pattern::AllPort(vec![Port::HTTP, Port::TELNET]) // HTTP + Telnet
            ])
        ])
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Switch>));
