use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::definitions::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UptimeKuma;

impl ServiceDefinition for UptimeKuma {
    fn name(&self) -> &'static str {
        "UptimeKuma"
    }
    fn description(&self) -> &'static str {
        "Self-hosted uptime monitoring tool"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "Uptime Kuma")
    }

    fn icon(&self) -> &'static str {
        "uptime-kuma"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<UptimeKuma>));
