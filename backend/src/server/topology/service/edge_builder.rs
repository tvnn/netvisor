use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashMap;
use uuid::Uuid;

use crate::server::{
    groups::types::GroupType,
    subnets::types::base::{Subnet, SubnetType},
    topology::{
        service::context::TopologyContext,
        types::{
            edges::{Edge, EdgeHandle, EdgeType},
            nodes::Node,
        },
    },
};

pub struct EdgeBuilder;

impl EdgeBuilder {
    /// Create group edges (connecting services in a group's service chain)
    pub fn create_group_edges(ctx: &TopologyContext) -> Vec<Edge> {
        ctx.groups
            .iter()
            .flat_map(|group| {
                let bindings = &group.base.service_bindings;
                bindings.windows(2).filter_map(|window| {
                    let interface_0 = ctx.services.iter().find_map(|s| {
                        if s.id == window[0].service_id {
                            if let Some(binding) = s.get_binding(window[0].binding_id) {
                                return Some(binding.interface_id());
                            }
                        }
                        None
                    });
                    let interface_1 = ctx.services.iter().find_map(|s| {
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
                        let source_subnet = ctx.get_subnet_from_interface_id(interface_0)?;
                        let target_subnet = ctx.get_subnet_from_interface_id(interface_1)?;

                        let source_is_infra = ctx
                            .get_interfaces_with_infra_service(source_subnet)
                            .contains(&Some(interface_0));
                        let target_is_infra = ctx
                            .get_interfaces_with_infra_service(target_subnet)
                            .contains(&Some(interface_1));

                        // Check if infra constraints are actually necessary
                        let source_needs_infra_constraint =
                            Self::subnet_has_mixed_infra(ctx, source_subnet);
                        let target_needs_infra_constraint =
                            Self::subnet_has_mixed_infra(ctx, target_subnet);

                        let (source_handle, target_handle) = EdgeHandle::from_subnet_layers(
                            source_subnet,
                            target_subnet,
                            source_is_infra && source_needs_infra_constraint,
                            target_is_infra && target_needs_infra_constraint,
                        );
                        
                        // Don't label edges if they are within a subnet (avoid clutter) or between subnets that have been
                        // consolidated (ie docker bridge subnets)
                        let label = if source_subnet == target_subnet
                            || (source_subnet.base.subnet_type == SubnetType::DockerBridge
                                && target_subnet.base.subnet_type == SubnetType::DockerBridge
                                && group.base.group_type == GroupType::VirtualizationHost)
                        {
                            None
                        } else {
                            Some(group.base.name.to_string())
                        };

                        return Some(Edge {
                            source: interface_0,
                            target: interface_1,
                            edge_type: EdgeType::Group(group.base.group_type),
                            label,
                            source_handle,
                            target_handle,
                        });
                    }
                    None
                })
            })
            .collect()
    }

    /// Create interface edges (connecting multiple interfaces on the same host)
    pub fn create_interface_edges(ctx: &TopologyContext) -> Vec<Edge> {
        ctx.hosts
            .iter()
            .flat_map(|host| {
                if let Some(origin_interface) = host.base.interfaces.first() {
                    host.base
                        .interfaces
                        .iter()
                        .filter(|interface| interface.id != origin_interface.id)
                        .filter_map(|interface| {
                            let source_subnet =
                                ctx.get_subnet_by_id(origin_interface.base.subnet_id)?;
                            let target_subnet = ctx.get_subnet_by_id(interface.base.subnet_id)?;

                            let source_is_infra = ctx
                                .get_interfaces_with_infra_service(source_subnet)
                                .contains(&Some(origin_interface.id));
                            let target_is_infra = ctx
                                .get_interfaces_with_infra_service(target_subnet)
                                .contains(&Some(interface.id));

                            // Skip edges to interfaces on docker bridge subnets; this connection is represented with an automatically created Group instead
                            if target_subnet.base.subnet_type == SubnetType::DockerBridge
                                || source_subnet.base.subnet_type == SubnetType::DockerBridge
                            {
                                return None;
                            }

                            // Check if infra constraints are actually necessary
                            let source_needs_infra_constraint =
                                Self::subnet_has_mixed_infra(ctx, source_subnet);
                            let target_needs_infra_constraint =
                                Self::subnet_has_mixed_infra(ctx, target_subnet);

                            let (source_handle, target_handle) = EdgeHandle::from_subnet_layers(
                                source_subnet,
                                target_subnet,
                                source_is_infra && source_needs_infra_constraint,
                                target_is_infra && target_needs_infra_constraint,
                            );

                            Some(Edge {
                                source: origin_interface.id,
                                target: interface.id,
                                edge_type: EdgeType::Interface,
                                label: Some(host.base.name.to_string()),
                                source_handle,
                                target_handle,
                            })
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    /// Check if a subnet has both infra and non-infra nodes
    /// If it only has one type, infra constraints are not necessary
    fn subnet_has_mixed_infra(ctx: &TopologyContext, subnet: &Subnet) -> bool {
        let infra_interfaces = ctx.get_interfaces_with_infra_service(subnet);

        // Get all interfaces in this subnet
        let all_interfaces_in_subnet: Vec<Uuid> = ctx
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

    /// Add edges to a petgraph Graph
    pub fn add_edges_to_graph(
        graph: &mut Graph<Node, Edge>,
        node_indices: &HashMap<Uuid, NodeIndex>,
        edges: Vec<Edge>,
    ) {
        for edge in edges {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                node_indices.get(&edge.source),
                node_indices.get(&edge.target),
            ) {
                graph.add_edge(src_idx, tgt_idx, edge);
            }
        }
    }
}
