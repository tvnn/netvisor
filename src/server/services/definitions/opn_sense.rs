use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpnSense;

impl ServiceDefinition for OpnSense {
    fn name(&self) -> &'static str { "OPNsense" }
    fn description(&self) -> &'static str { "Open-source firewall and routing platform" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkSecurity }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "OPNSense")
    }

    fn is_gateway(&self) -> bool { true }    
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpnSense>));
