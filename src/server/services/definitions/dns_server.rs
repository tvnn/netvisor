use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DnsServer;

impl ServiceDefinition for DnsServer {
    fn name(&self) -> &'static str { "Dns Server" }
    fn description(&self) -> &'static str { "A generic Dns server" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Unknown }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(Port::DNS_UDP, Port::DNS_TCP))
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<DnsServer>));
