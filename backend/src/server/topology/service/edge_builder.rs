use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashMap;
use uuid::Uuid;

use crate::server::{
    subnets::types::base::Subnet,
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
                        if let Some(binding) = s.get_binding(window[0]) {
                            return Some(binding.interface_id());
                        }
                        None
                    });
                    let interface_1 = ctx.services.iter().find_map(|s| {
                        if let Some(binding) = s.get_binding(window[1]) {
                            return Some(binding.interface_id());
                        }
                        None
                    });

                    if let (Some(Some(interface_0)), Some(Some(interface_1))) =
                        (interface_0, interface_1)
                    {
                        let (source_handle, target_handle) =
                            EdgeBuilder::determine_interface_handles(
                                ctx,
                                &interface_0,
                                &interface_1,
                            )?;

                        return Some(Edge {
                            source: interface_0,
                            target: interface_1,
                            edge_type: EdgeType::Group(group.base.group_type),
                            label: Some(group.base.name.to_string()),
                            source_handle,
                            target_handle,
                        });
                    }
                    None
                })
            })
            .collect()
    }

    pub fn create_containerized_service_edges(ctx: &TopologyContext) -> Vec<Edge> {
        ctx.services
            .iter()
            .filter(|s| !s.base.containers.is_empty())
            .filter_map(|s| {
                let host = ctx.get_host_by_id(s.base.host_id)?;

                let container_subnets: Vec<Uuid> = host
                    .base
                    .interfaces
                    .iter()
                    .filter_map(|i| ctx.get_subnet_by_id(i.base.subnet_id))
                    .filter_map(|s| {
                        if s.base.subnet_type.is_for_containers() {
                            return Some(s.id);
                        }
                        None
                    })
                    .collect();

                let container_subnet_interface_ids: Vec<Uuid> = host
                    .base
                    .interfaces
                    .iter()
                    .filter_map(|i| {
                        if container_subnets.contains(&i.base.subnet_id) {
                            return Some(i.id);
                        }
                        None
                    })
                    .collect();

                let origin_interface = host.base.interfaces.first()?;

                Some((s, origin_interface.id, container_subnet_interface_ids))
            })
            .flat_map(|(s, origin_interface_id, container_subnet_interface_ids)| {
                if let Some(host) = ctx.get_host_by_id(s.base.host_id) {
                    return s
                        .base
                        .containers
                        .iter()
                        .filter_map(move |cs| {
                            let containerized = ctx.get_service_by_id(*cs)?;

                            let container_binding_interface_id = containerized
                                .base
                                .bindings
                                .iter()
                                .filter_map(|b| b.interface_id())
                                .find(|i| container_subnet_interface_ids.contains(i))?;

                            let (source_handle, target_handle) =
                                EdgeBuilder::determine_interface_handles(
                                    ctx,
                                    &origin_interface_id,
                                    &container_binding_interface_id,
                                )?;

                            Some(Edge {
                                source: origin_interface_id,
                                target: container_binding_interface_id,
                                edge_type: EdgeType::ServiceVirtualization,
                                label: Some(format!("{} on {}", s.base.name, host.base.name)),
                                source_handle,
                                target_handle,
                            })
                        })
                        .collect();
                };
                Vec::new()
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
                                ctx.get_subnet_by_id(origin_interface.base.subnet_id);
                            let target_subnet = ctx.get_subnet_by_id(interface.base.subnet_id);

                            if let Some(source_subnet) = source_subnet {
                                if source_subnet.base.subnet_type.is_for_containers() {
                                    return None;
                                }
                            }

                            if let Some(target_subnet) = target_subnet {
                                if target_subnet.base.subnet_type.is_for_containers() {
                                    return None;
                                }
                            }

                            let (source_handle, target_handle) =
                                EdgeBuilder::determine_interface_handles(
                                    ctx,
                                    &origin_interface.id,
                                    &interface.id,
                                )?;

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

    /// Figure out handles for two interfaces
    pub fn determine_interface_handles(
        ctx: &TopologyContext,
        source_interface_id: &Uuid,
        target_interface_id: &Uuid,
    ) -> Option<(EdgeHandle, EdgeHandle)> {
        let source_subnet = ctx.get_subnet_from_interface_id(*source_interface_id)?;
        let target_subnet = ctx.get_subnet_from_interface_id(*target_interface_id)?;

        let source_is_infra = ctx
            .get_interfaces_with_infra_service(source_subnet)
            .contains(&Some(*source_interface_id));
        let target_is_infra = ctx
            .get_interfaces_with_infra_service(target_subnet)
            .contains(&Some(*target_interface_id));

        // Check if infra constraints are actually necessary
        let source_needs_infra_constraint = Self::subnet_has_mixed_infra(ctx, source_subnet);
        let target_needs_infra_constraint = Self::subnet_has_mixed_infra(ctx, target_subnet);

        Some(EdgeHandle::from_subnet_layers(
            source_subnet,
            target_subnet,
            source_is_infra && source_needs_infra_constraint,
            target_is_infra && target_needs_infra_constraint,
        ))
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
