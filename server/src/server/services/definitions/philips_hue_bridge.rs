use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::{Pattern, Vendor};
use crate::server::services::types::types::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PhilipsHueBridge;

impl ServiceDefinition for PhilipsHueBridge {
    fn name(&self) -> &'static str {
        "Philips Hue Bridge"
    }
    fn description(&self) -> &'static str {
        "Philips Hue Bridge for lighting control"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::PHILIPS),
            Pattern::WebService("/", "hue"),
        ])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<PhilipsHueBridge>
));
