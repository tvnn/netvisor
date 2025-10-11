use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::{
    service::context::TopologyContext,
    types::{
        edges::{Edge, EdgeHandle, EdgeType},
        nodes::Node,
    },
};

pub struct EdgeBuilder;

impl EdgeBuilder {
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

                            let (source_handle, target_handle) = EdgeHandle::from_subnet_layers(
                                source_subnet,
                                target_subnet,
                                source_is_infra,
                                target_is_infra,
                            );

                            // Apply infra constraints:
                            // - Infra interfaces can't have Right handles
                            // - Non-infra interfaces can't have Left handles
                            let source_handle =
                                Self::adjust_handle_for_infra(source_handle, source_is_infra);
                            let target_handle =
                                Self::adjust_handle_for_infra(target_handle, target_is_infra);

                            Some(Edge {
                                source: origin_interface.id,
                                target: interface.id,
                                edge_type: EdgeType::Interface,
                                label: host.base.name.to_string(),
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

                        let (source_handle, target_handle) = EdgeHandle::from_subnet_layers(
                            source_subnet,
                            target_subnet,
                            source_is_infra,
                            target_is_infra,
                        );

                        // Apply infra constraints
                        let source_handle =
                            Self::adjust_handle_for_infra(source_handle, source_is_infra);
                        let target_handle =
                            Self::adjust_handle_for_infra(target_handle, target_is_infra);

                        return Some(Edge {
                            source: interface_0,
                            target: interface_1,
                            edge_type: EdgeType::Group,
                            label: group.base.name.to_string(),
                            source_handle,
                            target_handle,
                        });
                    }
                    None
                })
            })
            .collect()
    }

    /// Adjust edge handles to respect infra constraints
    /// - Infra interfaces: Right -> Bottom (can't use Right)
    /// - Non-infra interfaces: Left -> Bottom (can't use Left)
    fn adjust_handle_for_infra(handle: EdgeHandle, is_infra: bool) -> EdgeHandle {
        match (&handle, is_infra) {
            // Infra can't use Right
            (EdgeHandle::Right, true) => EdgeHandle::Bottom,
            // Non-infra can't use Left
            (EdgeHandle::Left, false) => EdgeHandle::Bottom,
            // All other cases are fine
            _ => handle,
        }
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
