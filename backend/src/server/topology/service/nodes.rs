use crate::server::{
    groups::types::Group,
    hosts::types::base::Host,
    services::types::base::Service,
    subnets::types::base::{Subnet, SubnetType},
    topology::{
        service::{edges::TopologyEdgePlanner, utils::TopologyUtils},
        types::{
            base::{Ixy, SubnetLayout, Uxy},
            edges::{EdgeHandle, EdgeInfo},
            nodes::{Node, NodeType, SubnetChild, SubnetChildNodeSize},
        },
    },
};
use itertools::Itertools;
use std::collections::HashMap;
use uuid::Uuid;

const SUBNET_PADDING: Uxy = Uxy { x: 75, y: 75 };
const NODE_PADDING: Uxy = Uxy { x: 50, y: 50 };

pub struct TopologyNodePlanner {
    utils: TopologyUtils,
    edge_planner: TopologyEdgePlanner,
    no_subnet_id: Uuid,
}

impl Default for TopologyNodePlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl TopologyNodePlanner {
    pub fn new() -> Self {
        Self {
            utils: TopologyUtils::new(),
            edge_planner: TopologyEdgePlanner::new(),
            no_subnet_id: Uuid::new_v4(),
        }
    }

    pub fn create_subnet_child_nodes(
        &self,
        hosts: &[Host],
        subnets: &[Subnet],
        services: &[Service],
        groups: &[Group],
    ) -> (HashMap<Uuid, SubnetLayout>, Vec<Node>) {
        let children_by_subnet = self.group_children_by_subnet(hosts, services, groups, subnets);
        let mut child_nodes = Vec::new();

        let subnet_sizes: HashMap<Uuid, SubnetLayout> = children_by_subnet
            .iter()
            .map(|(subnet_id, children)| {
                let (size, infra_width) = self.calculate_subnet_size(
                    *subnet_id,
                    children,
                    hosts,
                    subnets,
                    services,
                    &mut child_nodes,
                );
                (*subnet_id, SubnetLayout { size, infra_width })
            })
            .collect();

        (subnet_sizes, child_nodes)
    }

    pub fn analyze_child_anchors(
        &self,
        child_id: Uuid,
        edge_infos: &[EdgeInfo],
    ) -> (Option<EdgeHandle>, usize) {
        // Find all edges involving this child
        let child_edges: Vec<_> = edge_infos
            .iter()
            .filter(|edge| edge.source_id == child_id || edge.target_id == child_id)
            .collect();

        let total_edges = child_edges.len();

        if child_edges.is_empty() {
            return (None, 0);
        }

        // Count anchors by handle direction
        let mut handle_counts: HashMap<EdgeHandle, usize> = HashMap::new();

        for edge in child_edges {
            // Determine which handle applies to this child
            let (source_handle, target_handle) =
                EdgeHandle::from_subnet_layers(edge.source_subnet, edge.target_subnet);
            let relevant_handle = if edge.source_id == child_id {
                &source_handle
            } else {
                &target_handle
            };

            *handle_counts.entry(relevant_handle.clone()).or_insert(0) += 1;
        }

        // Return the handle with the most anchors
        let primary_handle = handle_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(handle, _)| handle);

        (primary_handle, total_edges)
    }

    pub fn group_children_by_subnet(
        &self,
        hosts: &[Host],
        services: &[Service],
        groups: &[Group],
        subnets: &[Subnet],
    ) -> HashMap<Uuid, Vec<SubnetChild>> {
        let interface_edges = self.edge_planner.get_interface_edge_info(hosts, subnets);
        let group_edges = self
            .edge_planner
            .get_group_edge_info(groups, hosts, subnets, services);
        let all_edges: Vec<_> = interface_edges.into_iter().chain(group_edges).collect();

        hosts
            .iter()
            .flat_map(|host| {
                if !host.base.interfaces.is_empty() {
                    host.base
                        .interfaces
                        .iter()
                        .map(|interface| {
                            let services: Vec<Uuid> = services
                                .iter()
                                .filter_map(|s| {
                                    let interface_ids: Vec<Uuid> = s
                                        .base
                                        .bindings
                                        .iter()
                                        .filter_map(|b| b.interface_id())
                                        .collect();

                                    // Length comparison - if there is an binding with None as interface ID, meaning it listens on all interfaces for that port
                                    if interface_ids.contains(&interface.id)
                                        || interface_ids.len() < s.base.bindings.len()
                                    {
                                        Some(s.id)
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            let (primary_handle, anchor_count) =
                                self.analyze_child_anchors(interface.id, &all_edges);

                            let subnet_child = SubnetChild {
                                id: interface.id,
                                host_id: host.id,
                                interface_id: Some(interface.id),
                                size: SubnetChildNodeSize::from_service_count(services.len()),
                                primary_handle,
                                anchor_count,
                            };
                            (interface.base.subnet_id, subnet_child)
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                } else {
                    vec![(
                        self.no_subnet_id,
                        SubnetChild {
                            id: host.id,
                            host_id: host.id,
                            interface_id: None,
                            size: SubnetChildNodeSize::Small,
                            primary_handle: None,
                            anchor_count: 0,
                        },
                    )]
                    .into_iter()
                }
            })
            .fold(HashMap::new(), |mut acc, (subnet_id, child)| {
                acc.entry(subnet_id).or_default().push(child);
                acc
            })
    }

    pub fn calculate_subnet_size(
        &self,
        subnet_id: Uuid,
        children: &[SubnetChild],
        hosts: &[Host],
        subnets: &[Subnet],
        services: &[Service],
        child_nodes: &mut Vec<Node>,
    ) -> (Uxy, usize) {
        // Separate infrastructure from regular nodes
        let (infrastructure_children, regular_children) =
            if let Some(subnet) = subnets.iter().find(|s| s.id == subnet_id) {
                let infrastructure_host_ids: Vec<Uuid> = subnet
                    .get_dns_resolvers(hosts, services)
                    .iter()
                    .chain(&subnet.get_reverse_proxies(hosts, services))
                    .chain(&subnet.get_gateways(hosts, services))
                    .map(|service| service.base.host_id)
                    .collect();

                let (infrastructure, regular): (Vec<SubnetChild>, Vec<SubnetChild>) = children
                    .iter()
                    .sorted_by_key(|c| c.size.size().y)
                    .cloned()
                    .partition(|c| infrastructure_host_ids.contains(&c.host_id));

                (infrastructure, regular)
            } else {
                (Vec::new(), children.to_vec())
            };

        // Calculate regular nodes layout
        let (regular_grid_size, regular_grid_dimensions, regular_child_positions) = {
            let regular_grid_dimensions = self
                .utils
                .calculate_container_grid_dimensions(regular_children.len());

            let nearest_square_regular_child_grid_positions =
                self.utils.calculate_anchor_based_child_positions(
                    &regular_children,
                    &regular_grid_dimensions,
                );

            let (regular_child_positions, regular_grid_size) = self.utils.calculate_container_size(
                nearest_square_regular_child_grid_positions,
                &NODE_PADDING,
            );
            (
                regular_grid_size,
                regular_grid_dimensions,
                regular_child_positions,
            )
        };

        // Calculate infrastructure nodes layout
        let (infra_grid_size, infra_child_positions, infra_cols) = {
            // add cols for infra based on height of regular node grid
            let infra_cols = (infrastructure_children.len() as f64
                / regular_grid_dimensions.y as f64)
                .ceil() as usize;
            let infra_grid_dimensions = Uxy {
                x: infra_cols,
                y: regular_grid_dimensions.y,
            };

            let nearest_square_infra_child_grid_positions =
                self.utils.calculate_anchor_based_child_positions(
                    &infrastructure_children,
                    &infra_grid_dimensions,
                );

            let (infra_child_positions, infra_grid_size) = self
                .utils
                .calculate_container_size(nearest_square_infra_child_grid_positions, &NODE_PADDING);
            (infra_grid_size, infra_child_positions, infra_cols)
        };

        infrastructure_children.iter().for_each(|child| {
            if let Some(position) = infra_child_positions.get(&child.id) {
                child_nodes.push(Node {
                    id: child.id,
                    node_type: NodeType::HostNode {
                        subnet_id,
                        interface_id: child.interface_id,
                        host_id: child.host_id,
                        is_infra: true,
                    },
                    position: position.clone(),
                    size: child.size.size(),
                });
            }
        });

        regular_children.iter().for_each(|child| {
            if let Some(position) = regular_child_positions.get(&child.id) {
                // Shift position to the right if there are infra columns
                let node_position = Ixy {
                    x: position.x
                        + if infra_cols > 0 {
                            infra_grid_size.x as isize
                        } else {
                            0
                        },
                    y: position.y,
                };
                child_nodes.push(Node {
                    id: child.id,
                    node_type: NodeType::HostNode {
                        subnet_id,
                        interface_id: child.interface_id,
                        host_id: child.host_id,
                        is_infra: false,
                    },
                    position: node_position,
                    size: child.size.size(),
                });
            };
        });

        let total_size = Uxy {
            x: regular_grid_size.x + infra_grid_size.x,
            y: regular_grid_size.y.max(infra_grid_size.y),
        };

        (total_size, infra_grid_size.x)
    }

    pub fn create_subnet_nodes(
        &self,
        subnets: &[Subnet],
        layouts: &HashMap<Uuid, SubnetLayout>,
    ) -> Vec<Node> {
        let subnet_grid_positions = self
            .utils
            .calculate_subnet_grid_positions_by_layer(subnets, layouts);
        let (positions, _) = self
            .utils
            .calculate_container_size(subnet_grid_positions, &SUBNET_PADDING);

        layouts
            .iter()
            .filter_map(|(subnet_id, layout)| {
                if let Some(position) = positions.get(subnet_id) {
                    // Handle no_subnet case
                    if *subnet_id == self.no_subnet_id {
                        return Some(Node {
                            id: *subnet_id,
                            node_type: NodeType::SubnetNode {
                                infra_width: layout.infra_width,
                                subnet_type: SubnetType::None,
                            },
                            position: position.clone(),
                            size: layout.size.clone(),
                        });
                    }

                    // Handle regular subnet case
                    if let Some(subnet) = subnets.iter().find(|s| s.id == *subnet_id) {
                        return Some(Node {
                            id: *subnet_id,
                            node_type: NodeType::SubnetNode {
                                infra_width: layout.infra_width,
                                subnet_type: subnet.base.subnet_type.clone(),
                            },
                            position: position.clone(),
                            size: layout.size.clone(),
                        });
                    }
                }
                None
            })
            .collect()
    }
}
