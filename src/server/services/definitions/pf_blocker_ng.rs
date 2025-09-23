use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PfBlockerNg;

impl ServiceDefinition for PfBlockerNg {
    fn name(&self) -> &'static str { "pfBlockerNG" }
    fn description(&self) -> &'static str { "pfSense package for DNS/IP blocking" }
    fn category(&self) -> ServiceCategory { ServiceCategory::AdBlock }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/pfblockerng", "pfBlockerNG")))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PfBlockerNg>));
