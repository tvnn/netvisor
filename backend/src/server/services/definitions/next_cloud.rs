use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NextCloud;

impl ServiceDefinition for NextCloud {
    fn name(&self) -> &'static str {
        "NextCloud"
    }
    fn description(&self) -> &'static str {
        "Self-hosted cloud storage and collaboration platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Web
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::WebService("/", "Nextcloud")
    }

    fn icon(&self) -> &'static str {
        "nextcloud"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NextCloud>));
