use uuid::Uuid;

use crate::server::{
    groups::types::Group, hosts::types::base::Host, services::types::base::Service,
    subnets::types::base::Subnet,
};

/// Central context for topology building operations
/// Reduces parameter passing and provides helper methods
pub struct TopologyContext<'a> {
    pub hosts: &'a [Host],
    pub subnets: &'a [Subnet],
    pub services: &'a [Service],
    pub groups: &'a [Group],
}

impl<'a> TopologyContext<'a> {
    pub fn new(
        hosts: &'a [Host],
        subnets: &'a [Subnet],
        services: &'a [Service],
        groups: &'a [Group],
    ) -> Self {
        Self {
            hosts,
            subnets,
            services,
            groups,
        }
    }

    pub fn get_subnet_by_id(&self, subnet_id: Uuid) -> Option<&'a Subnet> {
        self.subnets.iter().find(|s| s.id == subnet_id)
    }

    pub fn get_host_by_id(&self, host_id: Uuid) -> Option<&'a Host> {
        self.hosts.iter().find(|h| h.id == host_id)
    }

    pub fn get_service_by_id(&self, service_id: Uuid) -> Option<&'a Service> {
        self.services.iter().find(|s| s.id == service_id)
    }

    pub fn get_services_bound_to_interface(&self, interface_id: Uuid) -> Vec<&'a Service> {
        self.services
            .iter()
            .filter(|s| s.to_bound_interface_ids().contains(&interface_id))
            .collect()
    }

    pub fn get_subnet_from_interface_id(&self, interface_id: Uuid) -> Option<&'a Subnet> {
        let interface = self
            .hosts
            .iter()
            .find_map(|h| h.base.interfaces.iter().find(|i| i.id == interface_id))?;

        self.get_subnet_by_id(interface.base.subnet_id)
    }

    pub fn get_host_from_interface_id(&self, interface_id: Uuid) -> Option<&'a Host> {
        self.hosts
            .iter()
            .find(|h| h.base.interfaces.iter().any(|i| i.id == interface_id))
    }

    pub fn get_interfaces_with_infra_service(&self, subnet: &Subnet) -> Vec<Option<Uuid>> {
        subnet
            .get_infra_services(self.hosts, self.services)
            .into_iter()
            .flat_map(|s| s.base.bindings.iter().map(|b| b.interface_id()))
            .collect()
    }
}
