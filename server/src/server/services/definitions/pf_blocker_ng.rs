use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::definitions::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PfBlockerNg;

impl ServiceDefinition for PfBlockerNg {
    fn name(&self) -> &'static str {
        "pfBlockerNG"
    }
    fn description(&self) -> &'static str {
        "pfSense package for DNS/IP blocking"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::AdBlock
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::AllPort(vec![PortBase::DnsTcp, PortBase::DnsUdp]),
            Pattern::WebService("/pfblockerng", "pfBlockerNG"),
        ])
    }

    fn icon(&self) -> &'static str {
        "pfsense"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PfBlockerNg>));
