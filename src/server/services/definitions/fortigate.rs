use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Fortigate;

impl ServiceDefinition for Fortigate {
    fn name(&self) -> &'static str { "Fortigate" }
    fn description(&self) -> &'static str { "Fortigate security appliance" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkSecurity }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "FortiGate")
    }

    fn is_gateway(&self) -> bool { true }    
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Fortigate>));
