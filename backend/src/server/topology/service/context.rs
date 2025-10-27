use std::collections::HashMap;

use uuid::Uuid;

use crate::server::{
    groups::types::Group,
    hosts::types::{base::Host, interfaces::Interface, virtualization::HostVirtualization},
    services::types::{
        base::Service, definitions::ServiceDefinitionExt, virtualization::ServiceVirtualization,
    },
    subnets::types::base::Subnet,
    topology::{
        service::optimizer::utils::OptimizerUtils,
        types::{
            api::TopologyRequestOptions,
            base::Ixy,
            edges::Edge,
            nodes::{Node, NodeType},
        },
    },
};

/// Composite quality score for graph layout
/// Lower scores are better (minimizing edge crossings and edge length)
#[derive(Debug, Clone, Copy)]
pub struct LayoutQuality {
    pub total_edge_length: f64,
    pub edge_crossings: usize,
    /// Weighted combination: crossings are heavily penalized
    /// Formula: (crossings * 10000) + edge_length
    /// This ensures that reducing crossings is prioritized over reducing edge length
    pub weighted_score: f64,
}

impl LayoutQuality {
    pub fn new(total_edge_length: f64, edge_crossings: usize) -> Self {
        // Crossings are weighted heavily (10000x) because they severely impact readability
        let weighted_score = (edge_crossings as f64 * 10000.0) + total_edge_length;
        Self {
            total_edge_length,
            edge_crossings,
            weighted_score,
        }
    }

    /// Returns true if this quality is better (lower score) than other
    pub fn is_better_than(&self, other: &LayoutQuality) -> bool {
        self.weighted_score < other.weighted_score
    }

    /// Returns the relative improvement as a percentage
    pub fn improvement_percentage(&self, previous: &LayoutQuality) -> f64 {
        if previous.weighted_score == 0.0 {
            return 0.0;
        }
        ((previous.weighted_score - self.weighted_score) / previous.weighted_score) * 100.0
    }
}

/// Central context for topology building operations
/// Reduces parameter passing and provides helper methods
pub struct TopologyContext<'a> {
    pub hosts: &'a [Host],
    pub subnets: &'a [Subnet],
    pub services: &'a [Service],
    pub groups: &'a [Group],
    pub options: &'a TopologyRequestOptions,
    utils: OptimizerUtils,
}

impl<'a> TopologyContext<'a> {
    pub fn new(
        hosts: &'a [Host],
        subnets: &'a [Subnet],
        services: &'a [Service],
        groups: &'a [Group],
        options: &'a TopologyRequestOptions,
    ) -> Self {
        Self {
            hosts,
            subnets,
            services,
            groups,
            options,
            utils: OptimizerUtils::new(),
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

    pub fn get_interface_by_id(&self, interface_id: Option<Uuid>) -> Option<&'a Interface> {
        self.hosts
            .iter()
            .find_map(|h| h.get_interface(&interface_id))
    }

    pub fn get_services_bound_to_interface(&self, interface_id: Uuid) -> Vec<&'a Service> {
        self.services
            .iter()
            .filter(|s| {
                s.to_bound_interface_ids()
                    .iter()
                    .any(|s| s.map(|id| id == interface_id).unwrap_or(false))
            })
            .collect()
    }

    pub fn get_node_subnet(&self, node_id: Uuid, nodes: &[Node]) -> Option<Uuid> {
        nodes
            .iter()
            .find(|n| n.id == node_id)
            .map(|node| match node.node_type {
                NodeType::InterfaceNode { subnet_id, .. } => subnet_id,
                NodeType::SubnetNode { .. } => node.id,
            })
    }

    pub fn edge_is_intra_subnet(&self, edge: &Edge) -> bool {
        if let (Some(source_subnet), Some(target_subnet)) = (
            self.get_subnet_from_interface_id(edge.source),
            self.get_subnet_from_interface_id(edge.target),
        ) {
            return source_subnet.id == target_subnet.id;
        }
        false
    }

    pub fn edge_is_multi_hop(
        &self,
        source_interface_id: &Uuid,
        target_interface_id: &Uuid,
    ) -> bool {
        if let (Some(source_subnet), Some(target_subnet)) = (
            self.get_subnet_from_interface_id(*source_interface_id),
            self.get_subnet_from_interface_id(*target_interface_id),
        ) {
            let vertical_order_difference = source_subnet.base.subnet_type.vertical_order()
                as isize
                - target_subnet.base.subnet_type.vertical_order() as isize;

            return vertical_order_difference.abs() > 1;
        }
        false
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

    pub fn get_host_is_virtualized_by(&self, host_id: &Uuid) -> Option<&Service> {
        if let Some(host) = self.get_host_by_id(*host_id)
            && let Some(HostVirtualization::Proxmox(proxmox_virtualization)) =
                &host.base.virtualization
        {
            return self
                .services
                .iter()
                .find(|s| s.id == proxmox_virtualization.service_id);
        }
        None
    }

    pub fn get_service_is_containerized_by(&self, service_id: &Uuid) -> Option<&Service> {
        if let Some(service) = self.get_service_by_id(*service_id)
            && let Some(ServiceVirtualization::Docker(docker_virtualization)) =
                &service.base.virtualization
        {
            return self
                .services
                .iter()
                .find(|s| s.id == docker_virtualization.service_id);
        }
        None
    }

    pub fn get_interfaces_with_infra_service(&self, subnet: &Subnet) -> Vec<Option<Uuid>> {
        self.services
            .iter()
            .filter(|s| {
                if let Some(host) = self.hosts.iter().find(|h| h.id == s.base.host_id) {
                    return (self
                        .options
                        .left_zone_service_categories
                        .contains(&s.base.service_definition.category())
                        || (self.options.show_gateway_in_left_zone
                            && s.base.service_definition.is_gateway()))
                        && subnet.has_interface_with_service(host, s);
                }
                false
            })
            .flat_map(|s| s.to_bound_interface_ids())
            .collect()
    }

    pub fn is_interface_infra(&self, interface_id: Uuid) -> bool {
        if let Some(subnet) = self.get_subnet_from_interface_id(interface_id) {
            let infra_interfaces = self.get_interfaces_with_infra_service(subnet);
            return infra_interfaces.contains(&Some(interface_id));
        }
        false
    }

    /// Check if a subnet has both infra and non-infra nodes
    /// If it only has one type, infra constraints are not necessary
    pub fn subnet_has_mixed_infra(&self, subnet: &Subnet) -> bool {
        let infra_interfaces = self.get_interfaces_with_infra_service(subnet);

        // Get all interfaces in this subnet
        let all_interfaces_in_subnet: Vec<Uuid> = self
            .hosts
            .iter()
            .flat_map(|h| &h.base.interfaces)
            .filter(|i| i.base.subnet_id == subnet.id)
            .map(|i| i.id)
            .collect();

        if all_interfaces_in_subnet.is_empty() {
            return false;
        }

        // Check if we have both infra and non-infra
        let has_infra = all_interfaces_in_subnet
            .iter()
            .any(|id| infra_interfaces.contains(&Some(*id)));

        let has_non_infra = all_interfaces_in_subnet
            .iter()
            .any(|id| !infra_interfaces.contains(&Some(*id)));

        has_infra && has_non_infra
    }

    /// Calculate overall quality score for the current graph layout
    /// This implements a composite quality metric combining:
    /// 1. Total edge length (aesthetic - shorter edges are cleaner)
    /// 2. Edge crossings (critical - crossings severely hurt readability)
    ///
    /// Used to guide optimization and detect convergence
    pub fn calculate_layout_quality(&self, nodes: &[Node], edges: &[Edge]) -> LayoutQuality {
        let total_edge_length = self.utils.calculate_total_edge_length(nodes, edges);
        let edge_crossings = self.count_edge_crossings(nodes, edges);

        LayoutQuality::new(total_edge_length, edge_crossings)
    }

    /// Count the number of edge crossings in the graph
    /// Uses geometric intersection detection on inter-subnet edges
    /// Each pair of intersecting edges counts as one crossing
    fn count_edge_crossings(&self, nodes: &[Node], edges: &[Edge]) -> usize {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, Node> = nodes.iter().map(|n| (n.id, n.clone())).collect();

        let inter_subnet_edges: Vec<&Edge> = edges
            .iter()
            .filter(|e| !self.edge_is_intra_subnet(e))
            .collect();

        self.utils
            .count_edge_crossings(&inter_subnet_edges, &node_map, &subnet_positions)
    }
}
