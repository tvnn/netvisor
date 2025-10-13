use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HpPrinter;

impl ServiceDefinition for HpPrinter {
    fn name(&self) -> &'static str {
        "Hp Printer"
    }
    fn description(&self) -> &'static str {
        "An HP Printer"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::HP),
            Pattern::AnyPort(vec![PortBase::Ipp, PortBase::LdpTcp, PortBase::LdpUdp]),
        ])
    }

    fn icon(&self) -> &'static str {
        "hp"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HpPrinter>));
