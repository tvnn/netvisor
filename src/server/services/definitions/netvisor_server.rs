use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NetvisorServer;

impl ServiceDefinition for NetvisorServer {
    fn name(&self) -> &'static str { "Netvisor Server" }
    fn description(&self) -> &'static str { "NetVisor server for network management" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Netvisor }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(Port::new_tcp(60072)))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NetvisorServer>));
