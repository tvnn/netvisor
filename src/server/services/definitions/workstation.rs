use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Workstation;

impl ServiceDefinition for Workstation {
    fn name(&self) -> &'static str { "Workstation" }
    fn description(&self) -> &'static str { "Desktop computer for productivity work" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Workstation }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllPort(vec!(Port::RDP, Port::SAMBA))
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Workstation>));
