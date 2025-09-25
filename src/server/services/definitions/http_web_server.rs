use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::{ServiceDefinition, ServiceDefinitionExt};
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HttpWebServer;

impl ServiceDefinition for HttpWebServer {
    fn name(&self) -> &'static str { "Http Web Server" }
    fn description(&self) -> &'static str { "A generic HTTP web server" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Web }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(
            Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPALT)),
            Pattern::AllMatchedService(|s| s.category() != ServiceCategory::Dashboard && !s.contains_web_service_pattern())
        ))
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HttpWebServer>));
