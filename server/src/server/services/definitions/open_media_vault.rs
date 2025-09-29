use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenMediaVault;

impl ServiceDefinition for OpenMediaVault {
    fn name(&self) -> &'static str {
        "OpenMediaVault"
    }
    fn description(&self) -> &'static str {
        "Debian-based NAS solution"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::AllPort(vec![PortBase::Samba]),
            Pattern::WebService("/", "OpenMediaVault"),
        ])
    }

    fn icon(&self) -> &'static str {
        "openmediavault"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OpenMediaVault>
));
