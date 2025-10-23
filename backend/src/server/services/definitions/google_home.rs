use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct GoogleHome;

impl ServiceDefinition for GoogleHome {
    fn name(&self) -> &'static str {
        "Google Home"
    }

    fn description(&self) -> &'static str {
        "Google Home smart speaker or display"
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
            Pattern::AllOf(vec![
                Pattern::Port(PortBase::new_tcp(8008)),
                Pattern::Port(PortBase::new_tcp(8009)),
            ]),
        ])
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "google-home"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<GoogleHome>));
