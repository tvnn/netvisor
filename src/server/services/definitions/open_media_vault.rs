use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenMediaVault;

impl ServiceDefinition for OpenMediaVault {
    fn name(&self) -> &'static str { "OpenMediaVault" }
    fn description(&self) -> &'static str { "Debian-based NAS solution" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Storage }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::SAMBA)),Pattern::WebService("/", "OpenMediaVault")))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpenMediaVault>));
