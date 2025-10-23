use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

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

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AllOf(vec![
                Pattern::Port(PortBase::DnsTcp),
                Pattern::Port(PortBase::DnsUdp),
            ]),
            Pattern::Endpoint(PortBase::Http, "/pfblockerng", "pfblockerng"),
        ])
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "pfsense"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PfBlockerNg>));
