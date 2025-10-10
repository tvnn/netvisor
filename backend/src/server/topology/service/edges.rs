use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashMap;
use uuid::Uuid;

use crate::server::{
    groups::types::Group,
    hosts::types::base::Host,
    services::types::base::Service,
    subnets::types::base::Subnet,
    topology::types::{
        edges::{Edge, EdgeInfo, EdgeType},
        nodes::Node,
    },
};
pub struct TopologyEdgePlanner {}

impl Default for TopologyEdgePlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl TopologyEdgePlanner {
    pub fn new() -> Self {
        Self {}
    }

    // Get interface edge information without node_indices
    pub fn get_interface_edge_info<'a>(
        &self,
        hosts: &[Host],
        subnets: &'a [Subnet],
    ) -> Vec<EdgeInfo<'a>> {
        hosts
            .iter()
            .flat_map(|host| {
                if let Some(origin_interface) = host.base.interfaces.first() {
                    host.base
                        .interfaces
                        .iter()
                        .filter(|interface| interface.id != origin_interface.id)
                        .filter_map(|interface| {
                            let source_subnet = subnets
                                .iter()
                                .find(|s| s.id == origin_interface.base.subnet_id)?;
                            let target_subnet =
                                subnets.iter().find(|s| s.id == interface.base.subnet_id)?;

                            Some(EdgeInfo {
                                source_id: origin_interface.id,
                                target_id: interface.id,
                                source_subnet,
                                target_subnet,
                                edge_type: EdgeType::Interface,
                                label: host.base.name.to_string(),
                            })
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    // Get group edge information without node_indices
    pub fn get_group_edge_info<'a>(
        &self,
        groups: &'a [Group],
        hosts: &'a [Host],
        subnets: &'a [Subnet],
        services: &'a [Service],
    ) -> Vec<EdgeInfo<'a>> {
        groups
            .iter()
            .flat_map(|group| {
                let bindings = &group.base.service_bindings;
                bindings.windows(2).filter_map(|window| {
                    let interface_0 = services.iter().find_map(|s| {
                        if s.id == window[0].service_id {
                            if let Some(binding) = s.get_binding(window[0].binding_id) {
                                return Some(binding.interface_id());
                            }
                        }
                        None
                    });
                    let interface_1 = services.iter().find_map(|s| {
                        if s.id == window[1].service_id {
                            if let Some(binding) = s.get_binding(window[1].binding_id) {
                                return Some(binding.interface_id());
                            }
                        }
                        None
                    });

                    if let (Some(Some(interface_0)), Some(Some(interface_1))) =
                        (interface_0, interface_1)
                    {
                        let source_subnet =
                            self.get_subnet_from_interface_id(&interface_0, hosts, subnets)?;
                        let target_subnet =
                            self.get_subnet_from_interface_id(&interface_1, hosts, subnets)?;

                        return Some(EdgeInfo {
                            source_id: interface_0,
                            target_id: interface_1,
                            source_subnet,
                            target_subnet,
                            edge_type: EdgeType::Group,
                            label: group.base.name.to_string(),
                        });
                    }
                    None
                })
            })
            .collect()
    }

    // Add edges to graph using EdgeInfo
    pub fn add_edges(
        &self,
        graph: &mut Graph<Node, Edge>,
        node_indices: &HashMap<Uuid, NodeIndex>,
        edge_infos: Vec<EdgeInfo>,
    ) {
        edge_infos
            .into_iter()
            .filter_map(|info| info.to_edge(node_indices))
            .for_each(|(src, tgt, edge)| {
                graph.add_edge(src, tgt, edge);
            });
    }

    // Convenience methods that combine getting info and adding to graph
    pub fn add_interface_edges(
        &self,
        graph: &mut Graph<Node, Edge>,
        node_indices: &HashMap<Uuid, NodeIndex>,
        hosts: &[Host],
        subnets: &[Subnet],
    ) {
        let edge_infos = self.get_interface_edge_info(hosts, subnets);
        self.add_edges(graph, node_indices, edge_infos);
    }

    pub fn add_group_edges(
        &self,
        graph: &mut Graph<Node, Edge>,
        node_indices: &HashMap<Uuid, NodeIndex>,
        groups: &[Group],
        hosts: &[Host],
        subnets: &[Subnet],
        services: &[Service],
    ) {
        let edge_infos = self.get_group_edge_info(groups, hosts, subnets, services);
        self.add_edges(graph, node_indices, edge_infos);
    }

    fn get_subnet_from_interface_id<'a>(
        &self,
        interface_id: &Uuid,
        hosts: &[Host],
        subnets: &'a [Subnet],
    ) -> Option<&'a Subnet> {
        if let Some(interface) = hosts
            .iter()
            .find_map(|h| h.base.interfaces.iter().find(|i| i.id == *interface_id))
        {
            return subnets.iter().find(|s| s.id == interface.base.subnet_id);
        }
        None
    }
}
