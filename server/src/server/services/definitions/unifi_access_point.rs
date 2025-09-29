use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::{Pattern, Vendor};
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UnifiAccessPoint;

impl ServiceDefinition for UnifiAccessPoint {
    fn name(&self) -> &'static str { "Unifi Access Point" }
    fn description(&self) -> &'static str { "Ubiquiti UniFi wireless access point" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkAccess }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::UBIQUITI), Pattern::WebService("/", "UniFi")))
    }

    fn icon(&self) -> &'static str {
        "unifi"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<UnifiAccessPoint>));
