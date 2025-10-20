use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NestThermostat;

impl ServiceDefinition for NestThermostat {
    fn name(&self) -> &'static str {
        "Nest Thermostat"
    }

    fn description(&self) -> &'static str {
        "Google Nest smart thermostat"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::NEST),
                Pattern::MacVendor(Vendor::GOOGLE),
            ]),
            Pattern::Port(PortBase::new_tcp(9543)),
        ])
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "google-home"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<NestThermostat>
));
