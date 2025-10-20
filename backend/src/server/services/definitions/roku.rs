use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RokuDevice;

impl ServiceDefinition for RokuDevice {
    fn name(&self) -> &'static str {
        "Roku Media Player"
    }

    fn description(&self) -> &'static str {
        "Roku streaming device or TV"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::ROKU),
            Pattern::Port(PortBase::new_tcp(8060)),
        ])
    }

    fn simple_icons_path(&self) -> &'static str {
        "roku"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<RokuDevice>));
