use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AdguardHome;

impl ServiceDefinition for AdguardHome {
    fn name(&self) -> &'static str {
        "Adguard Home"
    }
    fn description(&self) -> &'static str {
        "Network-wide ad and tracker blocking"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::AdBlock
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::AllPort(vec![PortBase::DnsUdp, PortBase::DnsTcp]),
            Pattern::WebService("/", "AdGuard Home"),
        ])
    }

    fn icon(&self) -> &'static str {
        "adguard-home"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<AdguardHome>));
