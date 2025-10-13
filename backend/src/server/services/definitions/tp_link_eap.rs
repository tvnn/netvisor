use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TpLinkEap;

impl ServiceDefinition for TpLinkEap {
    fn name(&self) -> &'static str {
        "TP-Link EAP"
    }
    fn description(&self) -> &'static str {
        "TP-Link EAP wireless access point"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::TPLINK),
            Pattern::WebService("/", "TP-LINK"),
        ])
    }

    fn icon(&self) -> &'static str {
        "tp-link"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TpLinkEap>));
