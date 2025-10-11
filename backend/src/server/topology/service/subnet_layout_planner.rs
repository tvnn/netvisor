use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::server::{
    shared::types::metadata::TypeMetadataProvider,
    subnets::types::base::SubnetType,
    topology::{
        service::{
            anchor_analyzer::AnchorAnalyzer, child_placement::ChildNodePlacement,
            context::TopologyContext, grid_calculator::GridCalculator,
        },
        types::{
            base::{Ixy, NodeLayout, SubnetLayout, Uxy},
            edges::{Edge, EdgeHandle},
            nodes::{Node, NodeType, SubnetChild, SubnetChildNodeSize},
        },
    },
};

const SUBNET_PADDING: Uxy = Uxy { x: 75, y: 75 };
const NODE_PADDING: Uxy = Uxy { x: 50, y: 50 };
const GROUP_DOCKER_BRIDGES_BY_HOST: bool = true;

pub struct SubnetLayoutPlanner {
    no_subnet_id: Uuid,
    handle_relocation_map: HashMap<Uuid, EdgeHandle>,
    consolidated_docker_subnets: HashMap<Uuid, Vec<Uuid>>,
}

impl Default for SubnetLayoutPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl SubnetLayoutPlanner {
    pub fn new() -> Self {
        Self {
            no_subnet_id: Uuid::new_v4(),
            handle_relocation_map: HashMap::new(),
            consolidated_docker_subnets: HashMap::new(),
        }
    }

    pub fn no_subnet_id(&self) -> Uuid {
        self.no_subnet_id
    }

    pub fn get_handle_relocation_map(&self) -> &HashMap<Uuid, EdgeHandle> {
        &self.handle_relocation_map
    }

    pub fn get_consolidated_docker_subnets(&self) -> &HashMap<Uuid, Vec<Uuid>> {
        &self.consolidated_docker_subnets
    }

    /// Main entry point: calculate subnet layouts and create all child nodes
    pub fn create_subnet_child_nodes(
        &mut self,
        ctx: &TopologyContext,
        all_edges: &[Edge],
    ) -> (HashMap<Uuid, SubnetLayout>, Vec<Node>) {
        let children_by_subnet = self.group_children_by_subnet(ctx, all_edges);
        let mut child_nodes = Vec::new();

        let subnet_sizes: HashMap<Uuid, SubnetLayout> = children_by_subnet
            .iter()
            .map(|(subnet_id, children)| {
                let (size, infra_width) =
                    self.calculate_subnet_size(*subnet_id, children, ctx, &mut child_nodes);
                (*subnet_id, SubnetLayout { size, infra_width })
            })
            .collect();

        (subnet_sizes, child_nodes)
    }

    /// Group host interfaces by subnet, with optional special handling for DockerBridge
    /// If GROUP_DOCKER_BRIDGES_BY_HOST is true, all DockerBridge interfaces for a given host
    /// are consolidated into one subnet
    fn group_children_by_subnet(
        &mut self,
        ctx: &TopologyContext,
        all_edges: &[Edge],
    ) -> HashMap<Uuid, Vec<SubnetChild>> {
        let mut children_by_subnet: HashMap<Uuid, Vec<SubnetChild>> = HashMap::new();

        // Track DockerBridge interfaces by host (only used if grouping is enabled)
        // Map: host_id -> (primary_subnet_id, Vec<(subnet_id, SubnetChild)>)
        let mut docker_by_host: HashMap<Uuid, (Uuid, Vec<(Uuid, SubnetChild)>)> = HashMap::new();

        for host in ctx.hosts {
            if host.base.interfaces.is_empty() {
                // No interfaces - add to no_subnet
                children_by_subnet
                    .entry(self.no_subnet_id)
                    .or_default()
                    .push(SubnetChild {
                        id: host.id,
                        host_id: host.id,
                        interface_id: None,
                        size: SubnetChildNodeSize::Small,
                        primary_handle: None,
                        anchor_count: 0,
                        should_relocate_handles: false,
                    });
                continue;
            }

            for interface in &host.base.interfaces {
                let subnet = ctx.get_subnet_by_id(interface.base.subnet_id);
                let subnet_type = subnet
                    .map(|s| s.base.subnet_type.clone())
                    .unwrap_or_default();

                let interface_bound_services: Vec<Uuid> = ctx
                    .services
                    .iter()
                    .filter_map(|s| {
                        let has_relevant_binding =
                            s.base.bindings.iter().any(|b| match b.interface_id() {
                                Some(binding_interface_id)
                                    if binding_interface_id == interface.id =>
                                {
                                    true
                                }
                                None => !subnet_type.is_internal(),
                                _ => false,
                            });

                        if has_relevant_binding {
                            Some(s.id)
                        } else {
                            None
                        }
                    })
                    .collect();

                if interface_bound_services.is_empty() {
                    continue;
                }

                let (primary_handle, anchor_count, should_relocate) =
                    AnchorAnalyzer::analyze_child_anchors(interface.id, all_edges, ctx);

                let child = SubnetChild {
                    id: interface.id,
                    host_id: host.id,
                    interface_id: Some(interface.id),
                    size: SubnetChildNodeSize::from_service_count(interface_bound_services.len()),
                    primary_handle,
                    anchor_count,
                    should_relocate_handles: should_relocate,
                };

                // Special handling for DockerBridge (only if grouping is enabled)
                if GROUP_DOCKER_BRIDGES_BY_HOST && matches!(subnet_type, SubnetType::DockerBridge) {
                    let entry = docker_by_host.entry(host.id).or_insert_with(|| {
                        // Use the first DockerBridge subnet we encounter for this host
                        (interface.base.subnet_id, Vec::new())
                    });
                    entry.1.push((interface.base.subnet_id, child));
                } else {
                    children_by_subnet
                        .entry(interface.base.subnet_id)
                        .or_default()
                        .push(child);
                }
            }
        }

        // Consolidate all DockerBridge children into their primary subnet (only if grouping is enabled)
        if GROUP_DOCKER_BRIDGES_BY_HOST {
            for (_host_id, (primary_subnet_id, docker_children_with_subnets)) in docker_by_host {
                if !docker_children_with_subnets.is_empty() {
                    // Track which subnets were consolidated
                    let mut consolidated_subnet_ids: Vec<Uuid> = docker_children_with_subnets
                        .iter()
                        .map(|(subnet_id, _)| *subnet_id)
                        .collect();

                    // Remove duplicates and sort for consistency
                    consolidated_subnet_ids.sort();
                    consolidated_subnet_ids.dedup();

                    // Store the consolidation mapping
                    self.consolidated_docker_subnets
                        .insert(primary_subnet_id, consolidated_subnet_ids);

                    // Add all children to the primary subnet
                    children_by_subnet
                        .entry(primary_subnet_id)
                        .or_default()
                        .extend(
                            docker_children_with_subnets
                                .into_iter()
                                .map(|(_, child)| child),
                        );
                }
            }
        }

        children_by_subnet
    }

    /// Calculate the size and layout of a subnet, creating child nodes
    fn calculate_subnet_size(
        &mut self,
        subnet_id: Uuid,
        children: &[SubnetChild],
        ctx: &TopologyContext,
        child_nodes: &mut Vec<Node>,
    ) -> (Uxy, usize) {
        // Separate infrastructure from regular nodes
        let (infrastructure_children, regular_children) =
            if let Some(subnet) = ctx.get_subnet_by_id(subnet_id) {
                let infrastructure_interface_ids = ctx.get_interfaces_with_infra_service(subnet);

                let (infrastructure, regular): (Vec<SubnetChild>, Vec<SubnetChild>) = children
                    .iter()
                    .sorted_by_key(|c| c.size.size().y)
                    .cloned()
                    .partition(|c| infrastructure_interface_ids.contains(&c.interface_id));

                (infrastructure, regular)
            } else {
                (Vec::new(), children.to_vec())
            };

        // Calculate regular nodes layout
        let (regular_grid_size, regular_grid_dimensions, regular_child_positions) = {
            let regular_grid_dimensions =
                GridCalculator::calculate_grid_dimensions(regular_children.len());

            let nearest_square_regular_child_grid_positions =
                ChildNodePlacement::calculate_anchor_based_positions(
                    &regular_children,
                    &regular_grid_dimensions,
                );

            let (regular_child_positions, regular_grid_size) =
                GridCalculator::calculate_container_size(
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
            let infra_cols = (infrastructure_children.len() as f64
                / regular_grid_dimensions.y as f64)
                .ceil() as usize;
            let infra_grid_dimensions = Uxy {
                x: infra_cols,
                y: regular_grid_dimensions.y,
            };

            let nearest_square_infra_child_grid_positions =
                ChildNodePlacement::calculate_anchor_based_positions(
                    &infrastructure_children,
                    &infra_grid_dimensions,
                );

            let (infra_child_positions, infra_grid_size) = GridCalculator::calculate_container_size(
                nearest_square_infra_child_grid_positions,
                &NODE_PADDING,
            );
            (infra_grid_size, infra_child_positions, infra_cols)
        };

        // Create infrastructure nodes
        infrastructure_children.iter().for_each(|child| {
            if let Some(position) = infra_child_positions.get(&child.id) {
                if child.should_relocate_handles {
                    if let Some(handle) = &child.primary_handle {
                        self.handle_relocation_map.insert(child.id, handle.clone());
                    }
                }

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

        // Create regular nodes
        regular_children.iter().for_each(|child| {
            if let Some(position) = regular_child_positions.get(&child.id) {
                if child.should_relocate_handles {
                    if let Some(handle) = &child.primary_handle {
                        self.handle_relocation_map.insert(child.id, handle.clone());
                    }
                }

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

    /// Create subnet container nodes with calculated positions
    pub fn create_subnet_nodes(
        &self,
        ctx: &TopologyContext,
        layouts: &HashMap<Uuid, SubnetLayout>,
    ) -> Vec<Node> {
        let subnet_grid_positions = self.calculate_subnet_grid_positions_by_layer(ctx, layouts);
        let (positions, _) =
            GridCalculator::calculate_container_size(subnet_grid_positions, &SUBNET_PADDING);

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
                                label_override: None,
                            },
                            position: position.clone(),
                            size: layout.size.clone(),
                        });
                    }

                    if let Some(consolidated_subnet_ids) =
                        self.consolidated_docker_subnets.get(subnet_id)
                    {
                        let label_override = SubnetType::DockerBridge.name().to_owned()
                            + ": ("
                            + &ctx
                                .subnets
                                .iter()
                                .filter(|s| consolidated_subnet_ids.contains(&s.id))
                                .map(|s| s.base.cidr.to_string())
                                .join(", ")
                            + ")";

                        return Some(Node {
                            id: *subnet_id,
                            node_type: NodeType::SubnetNode {
                                infra_width: layout.infra_width,
                                subnet_type: SubnetType::DockerBridge,
                                label_override: Some(label_override),
                            },
                            position: position.clone(),
                            size: layout.size.clone(),
                        });
                    }

                    // Handle regular subnet case
                    if let Some(subnet) = ctx.get_subnet_by_id(*subnet_id) {
                        return Some(Node {
                            id: *subnet_id,
                            node_type: NodeType::SubnetNode {
                                infra_width: layout.infra_width,
                                subnet_type: subnet.base.subnet_type.clone(),
                                label_override: None,
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

    /// Calculate positions of subnets given layer values
    fn calculate_subnet_grid_positions_by_layer(
        &self,
        ctx: &TopologyContext,
        layouts: &HashMap<Uuid, SubnetLayout>,
    ) -> Vec<Vec<(Uuid, NodeLayout)>> {
        let sorted: Vec<_> = ctx
            .subnets
            .iter()
            .sorted_by_key(|s| {
                (
                    s.base.subnet_type.default_layer(),
                    s.base.subnet_type.layer_priority(),
                    s.base.name.clone(),
                )
            })
            .filter_map(|s| layouts.get(&s.id).map(|layout| (s, layout)))
            .collect();

        let mut subnets_by_layer: BTreeMap<usize, Vec<(&Uuid, &SubnetLayout)>> = BTreeMap::new();
        for (subnet, layout) in sorted {
            subnets_by_layer
                .entry(subnet.base.subnet_type.default_layer())
                .or_default()
                .push((&subnet.id, layout));
        }

        subnets_by_layer
            .into_iter()
            .enumerate()
            .map(|(row_index, (_layer, row))| {
                row.into_iter()
                    .enumerate()
                    .map(|(y, (id, layout))| {
                        (
                            *id,
                            NodeLayout {
                                size: layout.size.clone(),
                                grid_position: Uxy { x: row_index, y },
                            },
                        )
                    })
                    .collect()
            })
            .collect()
    }
}
