use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};
use uuid::Uuid;

use crate::server::{
    hosts::types::base::Host,
    services::types::base::Service,
    shared::types::metadata::TypeMetadataProvider,
    subnets::types::base::SubnetType,
    topology::{
        service::{
            anchor_analyzer::ChildEdgeAnalyzer, child_placement::ChildNodePlacement,
            context::TopologyContext, grid_calculator::GridCalculator,
        },
        types::{
            base::{Ixy, NodeLayout, SubnetLayout, Uxy},
            edges::Edge,
            nodes::{Node, NodeType, SubnetChild},
        },
    },
};

pub const SUBNET_PADDING: Uxy = Uxy { x: 125, y: 125 };
pub const NODE_PADDING: Uxy = Uxy { x: 50, y: 50 };

pub struct SubnetLayoutPlanner {
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
            consolidated_docker_subnets: HashMap::new(),
        }
    }

    pub fn get_consolidated_docker_subnets(&self) -> &HashMap<Uuid, Vec<Uuid>> {
        &self.consolidated_docker_subnets
    }

    /// Main entry point: calculate subnet layouts and create all child nodes
    pub fn create_subnet_child_nodes(
        &mut self,
        ctx: &TopologyContext,
        all_edges: &mut [Edge],
        group_docker_bridges_by_host: bool,
        docker_bridge_host_subnet_id_to_group_on: HashMap<Uuid, Uuid>,
    ) -> (HashMap<Uuid, SubnetLayout>, Vec<Node>) {
        let children_by_subnet = self.group_children_by_subnet(
            ctx,
            all_edges,
            group_docker_bridges_by_host,
            docker_bridge_host_subnet_id_to_group_on,
        );
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

    fn determine_subnet_child_header_text(
        &self,
        ctx: &TopologyContext,
        interface_bound_services: &Vec<&Service>,
        host: &Host,
    ) -> Option<String> {
        if let Some(service) = ctx.get_host_is_virtualized_by(&host.id) {
            let host_interface_subnet_ids: Vec<Uuid> = host
                .base
                .interfaces
                .iter()
                .map(|i| i.base.subnet_id)
                .collect();
            let virtualization_service_interface_subnet_ids: Vec<Uuid> = service
                .base
                .bindings
                .iter()
                .filter_map(|b| ctx.get_interface_by_id(b.interface_id()))
                .map(|i| i.base.subnet_id)
                .collect();

            let host_interface_subnet_ids_hashset: HashSet<&Uuid> =
                host_interface_subnet_ids.iter().collect();
            let virtualization_service_interface_subnet_ids_hashset: HashSet<&Uuid> =
                virtualization_service_interface_subnet_ids.iter().collect();

            let intersection: Vec<&Uuid> = host_interface_subnet_ids_hashset
                .intersection(&virtualization_service_interface_subnet_ids_hashset)
                .cloned()
                .collect();

            // If they have one interface on a common subnet, and they both don't have multiple interfaces with the subnet
            // Use the IP address from that interface in the header text
            match intersection.first() {
                Some(first) if intersection.len() == 1 => {
                    if let Some(interface) = host
                        .base
                        .interfaces
                        .iter()
                        .find(|i| i.base.subnet_id == **first)
                    {
                        if host_interface_subnet_ids
                            .iter()
                            .filter(|i| i == first)
                            .count()
                            == 1
                            && virtualization_service_interface_subnet_ids
                                .iter()
                                .filter(|i| i == first)
                                .count()
                                == 1
                        {
                            return Some(format!(
                                "VM: {} @ {}",
                                service.base.name, interface.base.ip_address
                            ));
                        }
                    }
                    return Some(format!("VM: {}", service.base.name));
                }
                _ => return Some(format!("VM: {}", service.base.name)),
            }
        }

        let first_service_name_matches_host_name = match interface_bound_services.first() {
            Some(first_service) => first_service.base.name == host.base.name,
            None => false,
        };

        let host_has_name = host.base.name != "Unknown Device" && !host.base.name.is_empty();

        let interface_count = host.base.interfaces.len();

        if !first_service_name_matches_host_name && host_has_name && interface_count < 2 {
            return Some(host.base.name.clone());
        }

        None
    }

    /// Group host interfaces by subnet, with optional special handling for DockerBridge
    /// If group_docker_bridges_by_host is true, all DockerBridge interfaces for a given host
    /// are consolidated into one subnet
    fn group_children_by_subnet(
        &mut self,
        ctx: &TopologyContext,
        all_edges: &mut [Edge],
        group_docker_bridges_by_host: bool,
        docker_bridge_host_subnet_id_to_group_on: HashMap<Uuid, Uuid>,
    ) -> HashMap<Uuid, Vec<SubnetChild>> {
        let mut children_by_subnet: HashMap<Uuid, Vec<SubnetChild>> = HashMap::new();

        // Track DockerBridge interfaces by host (only used if grouping is enabled)
        // Map: (host_id, primary_subnet_id) -> Vec<subnet_id>)
        let mut docker_subnets_by_host: HashMap<(Uuid, Uuid), Vec<Uuid>> = HashMap::new();

        for host in ctx.hosts {
            for interface in &host.base.interfaces {
                let subnet = ctx.get_subnet_by_id(interface.base.subnet_id);
                let subnet_type = subnet.map(|s| s.base.subnet_type).unwrap_or_default();

                let interface_bound_services: Vec<&Service> = ctx
                    .services
                    .iter()
                    .filter(|s| {
                        s.base.bindings.iter().any(|b| match b.interface_id() {
                            Some(binding_interface_id) if binding_interface_id == interface.id => {
                                true
                            }
                            None => !subnet_type.is_internal(),
                            _ => false,
                        })
                    })
                    .collect();

                if interface_bound_services.is_empty() {
                    continue;
                }

                let edges = ChildEdgeAnalyzer::analyze_child_edges(interface.id, all_edges, ctx);

                let header_text =
                    self.determine_subnet_child_header_text(ctx, &interface_bound_services, host);

                let child = SubnetChild {
                    id: interface.id,
                    host_id: host.id,
                    size: Uxy::subnet_child_size_from_service_count(
                        interface_bound_services.len(),
                        header_text.is_some(),
                    ),
                    header: header_text,
                    interface_id: Some(interface.id),
                    edges,
                };

                // Special handling for DockerBridge (only if grouping is enabled)
                if group_docker_bridges_by_host && matches!(subnet_type, SubnetType::DockerBridge) {
                    if let Some(subnet_grouping_id) =
                        docker_bridge_host_subnet_id_to_group_on.get(&host.id)
                    {
                        docker_subnets_by_host
                            .entry((host.id, *subnet_grouping_id))
                            .or_default()
                            .push(interface.base.subnet_id);

                        children_by_subnet
                            .entry(*subnet_grouping_id)
                            .or_default()
                            .push(child);
                    }
                } else {
                    children_by_subnet
                        .entry(interface.base.subnet_id)
                        .or_default()
                        .push(child);
                }
            }
        }

        // Consolidate all DockerBridge children into their primary subnet (only if grouping is enabled)
        if group_docker_bridges_by_host {
            for ((_, grouping_id), mut subnet_ids) in docker_subnets_by_host {
                // Remove duplicates and sort for consistency
                subnet_ids.sort();
                subnet_ids.dedup();

                // Store the consolidation mapping
                self.consolidated_docker_subnets
                    .insert(grouping_id, subnet_ids);
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
                    .sorted_by_key(|c| c.size.y)
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
                    ctx,
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
                    ctx,
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
                child_nodes.push(Node {
                    id: child.id,
                    node_type: NodeType::HostNode {
                        subnet_id,
                        interface_id: child.interface_id,
                        host_id: child.host_id,
                        is_infra: true,
                        header: child.header.clone(),
                    },
                    position: *position,
                    size: child.size,
                });
            }
        });

        // Create regular nodes
        regular_children.iter().for_each(|child| {
            if let Some(position) = regular_child_positions.get(&child.id) {
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
                        header: child.header.clone(),
                    },
                    position: node_position,
                    size: child.size,
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
                    if let Some(consolidated_subnet_ids) =
                        self.consolidated_docker_subnets.get(subnet_id)
                    {
                        let header = SubnetType::DockerBridge.name().to_owned()
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
                                header: Some(header),
                            },
                            position: *position,
                            size: layout.size,
                        });
                    }

                    // Handle regular subnet case
                    if let Some(subnet) = ctx.get_subnet_by_id(*subnet_id) {
                        return Some(Node {
                            id: *subnet_id,
                            node_type: NodeType::SubnetNode {
                                infra_width: layout.infra_width,
                                subnet_type: subnet.base.subnet_type,
                                header: None,
                            },
                            position: *position,
                            size: layout.size,
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
                    s.base.subnet_type.vertical_order(),
                    s.base.subnet_type.horizontal_order(),
                    s.base.name.clone(),
                )
            })
            .filter_map(|s| layouts.get(&s.id).map(|layout| (s, layout)))
            .collect();

        let mut subnets_by_layer: BTreeMap<usize, Vec<(&Uuid, &SubnetLayout)>> = BTreeMap::new();
        for (subnet, layout) in sorted {
            subnets_by_layer
                .entry(subnet.base.subnet_type.vertical_order())
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
                                size: layout.size,
                                grid_position: Uxy { x: row_index, y },
                            },
                        )
                    })
                    .collect()
            })
            .collect()
    }
}
