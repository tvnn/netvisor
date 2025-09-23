use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Bind9;

impl ServiceDefinition for Bind9 {
    fn name(&self) -> &'static str { "Bind9" }
    fn description(&self) -> &'static str { "Berkeley Internet Name Domain DNS server" }
    fn category(&self) -> ServiceCategory { ServiceCategory::DNS }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllPort(vec!(
            Port::DNS_UDP, 
            Port::new_tcp(8053)
        ))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Bind9>));
