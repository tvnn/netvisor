use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RingDoorbell;

impl ServiceDefinition for RingDoorbell {
    fn name(&self) -> &'static str {
        "Ring Doorbell"
    }

    fn description(&self) -> &'static str {
        "Ring video doorbell or security camera"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::AMAZON),
            Pattern::AnyOf(vec![
                Pattern::Port(PortBase::new_tcp(8557)),
                Pattern::Port(PortBase::new_tcp(9998)),
                Pattern::Port(PortBase::new_tcp(19302)),
                Pattern::Port(PortBase::new_tcp(9999)),
            ]),
        ])
    }

    fn simple_icons_path(&self) -> &'static str {
        "ring"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<RingDoorbell>
));
