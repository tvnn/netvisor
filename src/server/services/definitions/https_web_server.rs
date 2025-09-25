use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::{ServiceDefinition, ServiceDefinitionExt};
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HttpsWebServer;

impl ServiceDefinition for HttpsWebServer {
    fn name(&self) -> &'static str { "Https Web Server" }
    fn description(&self) -> &'static str { "A generic HTTPS Web Server" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Web }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(
            Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTPSALT)),
            Pattern::AllMatchedService(|s| s.category() != ServiceCategory::Dashboard && !s.contains_web_service_pattern())
        ))
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HttpsWebServer>));
