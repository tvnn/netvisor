use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AdguardHome;

impl ServiceDefinition for AdguardHome {
    fn name(&self) -> &'static str { "Adguard Home" }
    fn description(&self) -> &'static str { "Network-wide ad and tracker blocking" }
    fn category(&self) -> ServiceCategory { ServiceCategory::AdBlock }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(
            Pattern::AllPort(vec!(Port::DNS_UDP, Port::DNS_TCP)), 
            Pattern::WebService("/", "AdGuard Home")
        ))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<AdguardHome>));
