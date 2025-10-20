use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FiosExtender;

impl ServiceDefinition for FiosExtender {
    fn name(&self) -> &'static str {
        "Fios Extender"
    }
    fn description(&self) -> &'static str {
        "Fios device providing mesh networking services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortBase::Http, "/#/login/", "fios"),
            Pattern::Not(&Pattern::IsGateway),
        ])
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "fios"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<FiosExtender>
));
