// backend/src/server/services/definitions/amazon_echo.rs
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AmazonEcho;

impl ServiceDefinition for AmazonEcho {
    fn name(&self) -> &'static str {
        "Amazon Echo"
    }

    fn description(&self) -> &'static str {
        "Amazon Echo smart speaker"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::AMAZON),
            Pattern::Port(PortBase::new_tcp(40317)),
        ])
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "alexa"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<AmazonEcho>));
