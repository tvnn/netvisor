use std::{collections::{HashMap}, sync::Arc};

use anyhow::Error;
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph};
use uuid::Uuid;

use crate::server::{
    groups::{service::GroupService, types::Group}, hosts::{service::HostService, types::{base::Host, targets::HostTarget}}, services::{service::ServiceService, types::base::Service}, subnets::{service::SubnetService, types::base::Subnet}, topology::types::base::{Edge, EdgeHandle, EdgeType, Node, NodeLayout, NodeType, SubnetChild, SubnetChildNodeSize, SubnetLayout, XY}
};

const SUBNET_PADDING: XY = XY{x: 75, y: 75};
const NODE_PADDING: XY = XY{x: 50, y: 50};

pub struct TopologyService {
    host_service: Arc<HostService>,
    subnet_service: Arc<SubnetService>,
    group_service: Arc<GroupService>,
    service_service: Arc<ServiceService>,
    no_subnet_id: Uuid,
    wan_id: Uuid
}

impl TopologyService {
    pub fn new(host_service: Arc<HostService>, subnet_service: Arc<SubnetService>, group_service: Arc<GroupService>, service_service: Arc<ServiceService>) -> Self {
        Self { 
            host_service,
            subnet_service,
            group_service,
            service_service,
            no_subnet_id: Uuid::new_v4(),
            wan_id: Uuid::new_v4()
        }
    }

    pub async fn build_graph(&self) -> Result<Graph<Node, Edge>, Error> {
        let hosts = self.host_service.get_all_hosts().await?;
        let subnets = self.subnet_service.get_all_subnets().await?;
        let groups = self.group_service.get_all_groups().await?;
        let services = self.service_service.get_all_services().await?;
                
        // First pass: create nodes with positions, determining layout from bottom up
        let (subnet_sizes, child_nodes) = self.create_subnet_child_nodes(&hosts, &subnets, &services);
        let subnet_nodes = self.create_subnet_nodes(&subnets, &subnet_sizes);

        // Add nodes to graph
        let mut graph: Graph<Node, Edge> = Graph::new();
        let node_indices: HashMap<Uuid, NodeIndex> = subnet_nodes
            .into_iter()
            .chain(child_nodes.into_iter())
            .map(|node| {
                let node_id = node.id;
                let node_idx = graph.add_node(node);
                (node_id, node_idx)
            })
            .collect();
        
        // Second pass: add edges
        self.add_group_edges(&mut graph, &node_indices, &groups);
        self.add_interface_edges(&mut graph, &node_indices, &hosts);
        
        Ok(graph)
    }

    // Helper methods for positioning nodes in grids of arbitrary dimensions (rows / cols) and size (pixels)

    /// Figure out closest shape to square that can contain children
    fn calculate_container_grid_dimensions(&self, children: usize) -> XY {
        if children == 0 {
            return XY { x: 1, y: 1 };
        }
        
        let x = (children as f64).sqrt().ceil() as usize;
        let y = ((children as f64) / x as f64).ceil() as usize;
        XY { x, y }
    }

    /// Calculate the coordinates of a child in a grid given its index
    fn calculate_child_coordinates_in_grid(&self, grid: &XY, child_idx: usize) -> XY {
        XY {
            x: child_idx % grid.x,
            y: ((child_idx / grid.x) as f64).floor() as usize
        }
    }

    /// Calculate the size of a container and positions of arbitrarily-sized children in that container
    fn calculate_container_size_and_child_positions(&self, child_sizes: Vec<(Uuid, XY)>, container_grid: &XY, padding: &XY) -> (HashMap<Uuid, XY>, XY) {
        let mut child_positions = HashMap::new();

        let mut rows: Vec<Vec<(Uuid, NodeLayout)>> = vec![Vec::new(); container_grid.y];

        for (idx, (id, size)) in child_sizes.into_iter().enumerate() {
            let grid_position = self.calculate_child_coordinates_in_grid(&container_grid, idx);
            rows[grid_position.y].push((id, NodeLayout {size, grid_position}));
        }

        let mut current_y = padding.y;
        let mut max_x = 0;
        let mut max_y = 0;

        for row in rows {
            if row.is_empty() {
                continue;
            }

            let mut current_x = padding.x;
            let mut max_height_in_row = 0;

            for (id, layout) in row {
                child_positions.insert(id, XY { x: current_x, y: current_y });
                current_x += layout.size.x + padding.x;
                max_height_in_row = max_height_in_row.max(layout.size.y);
            }

            current_y += max_height_in_row + padding.y;
            max_x = max_x.max(current_x);
            max_y = max_y.max(current_y);
        }

        let container_size = XY {
            x: max_x,
            y: max_y
        };

        (child_positions, container_size)
    }

    // 1st pass
    fn create_subnet_child_nodes(
        &self, 
        hosts: &[Host],
        subnets: &[Subnet],
        services: &[Service]
    ) -> (HashMap<Uuid, SubnetLayout>, Vec<Node>) {
        let children_by_subnet = self.group_children_by_subnet(hosts, services);
        let mut child_nodes = Vec::new();

        let subnet_sizes: HashMap<Uuid, SubnetLayout> = children_by_subnet.iter().map(|(subnet_id, children)| {
            let (size, infra_width) = self.calculate_subnet_size(*subnet_id, children, subnets, services, &mut child_nodes);
            (*subnet_id, SubnetLayout{
                size, infra_width
            })
        })
        .sorted_by_key(|(_, layout)| (layout.size.x*layout.size.y) )
        .collect();

        (subnet_sizes, child_nodes)
    }

    fn group_children_by_subnet(&self, hosts: &[Host], services: &[Service]) -> HashMap<Uuid, Vec<SubnetChild>> {
        hosts.iter()
            .flat_map(|host| {   

                if host.base.interfaces.len() > 0 {
                    return host.base.interfaces.iter()
                        .map(|interface| {

                            let services: Vec<Uuid> = services.iter()
                                .filter_map(|s| if s.base.interface_bindings.contains(&interface.id) {Some(s.id)} else {None})
                                .collect();

                            let subnet_child = SubnetChild {
                                id: interface.id,
                                host_id: host.id,
                                interface_id: Some(interface.id),
                                size: SubnetChildNodeSize::from_service_count(services.len()),
                            };
                            (interface.base.subnet_id, subnet_child)
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                } else {
                    let is_wan_node = match (host.base.target.clone(), host.base.interfaces.clone()) {
                        (HostTarget::ExternalIp(..), _) => true,
                        (HostTarget::Hostname, interfaces) => interfaces.len() == 0,
                        (_,_) => false
                    };
                    let id = if is_wan_node {self.wan_id} else {self.no_subnet_id};
                    return vec![(id, SubnetChild {
                            id: host.id,
                            host_id: host.id,
                            interface_id: None,
                            size: SubnetChildNodeSize::Small
                        })].into_iter()
                }
            })
            .fold(HashMap::new(), |mut acc, (subnet_id, child)| {
                acc.entry(subnet_id).or_insert_with(Vec::new).push(child);
                acc
            })
    }

    fn calculate_subnet_size(
        &self,
        subnet_id: Uuid,
        children: &[SubnetChild],
        subnets: &[Subnet],
        services: &[Service],
        child_nodes: &mut Vec<Node>
    ) -> (XY, usize) {
        // Separate infrastructure from regular nodes
        let (infrastructure_children, regular_children) = if let Some(subnet) = subnets.iter().find(|s| s.id == subnet_id) {
            let infrastructure_host_ids: Vec<Uuid> = subnet.base.gateways.iter()
                .chain(&subnet.base.dns_resolvers)
                .chain(&subnet.base.reverse_proxies)
                .filter_map(|id| services.iter().find(|s| s.id == *id))
                .map(|s| s.base.host_id)
                .collect();

            let infrastructure: Vec<_> = children
                .iter()
                .filter(|c| infrastructure_host_ids.contains(&c.host_id))
                .sorted_by_key(|c| c.size.size().y )
                .collect();
            
            let regular: Vec<_> = children
                .iter()
                .filter(|c| !infrastructure_host_ids.contains(&c.host_id))
                .sorted_by_key(|c| c.size.size().y )
                .collect();
            
            (infrastructure, regular)
        } else {
            (Vec::new(), children.iter().collect())
        };

        // Calculate regular nodes layout
        let (regular_grid_size, regular_grid_dimensions, regular_child_positions) = {
            let regular_grid_dimensions = self.calculate_container_grid_dimensions(regular_children.len());
            let (regular_child_positions, regular_grid_size) = self.calculate_container_size_and_child_positions(
                regular_children.iter().map(|c| (c.id, c.size.size())).collect::<Vec<_>>(),
                &regular_grid_dimensions,
                &NODE_PADDING
            );
            (regular_grid_size, regular_grid_dimensions, regular_child_positions)
        };

        // Calculate infrastructure nodes layout
        let (infra_grid_size, infra_child_positions, infra_cols) = {
            // add cols for infra based on height of regular node grid
            let infra_cols = (infrastructure_children.len() as f64 / regular_grid_dimensions.y as f64).ceil() as usize;
            let infra_grid_dimensions = XY { x: infra_cols, y: regular_grid_dimensions.y };

            let (infra_child_positions, infra_grid_size) = self.calculate_container_size_and_child_positions(
                infrastructure_children.iter().map(|c| (c.id, c.size.size())).collect::<Vec<_>>(),
                &infra_grid_dimensions,
                &NODE_PADDING
            );
            (infra_grid_size, infra_child_positions, infra_cols)
        };
        
        infrastructure_children.iter().for_each(|child| {
            if let Some(position) = infra_child_positions.get(&child.id) {
                child_nodes.push(Node { 
                    id: child.id,
                    interface_id: child.interface_id,
                    host_id: Some(child.host_id),
                    node_type: NodeType::HostNode,
                    parent_id: Some(subnet_id), 
                    position: position.clone(),
                    size: child.size.size(),
                    infra_width: None,
                    subnet_label: None
                });
            }
        });

        regular_children.iter().for_each(|child| {
            if let Some(position) = regular_child_positions.get(&child.id) {
                // Shift position to the right if there are infra columns
                let node_position = XY {
                    x: position.x + if infra_cols > 0 {infra_grid_size.x} else {0},
                    y: position.y
                };
                child_nodes.push(Node { 
                    id: child.id,
                    interface_id: child.interface_id,
                    host_id: Some(child.host_id),
                    node_type: NodeType::HostNode,
                    parent_id: Some(subnet_id), 
                    position: node_position,
                    size: child.size.size(),
                    infra_width: None,
                    subnet_label: None
                });
            };
        });

        let total_size = XY {
            x: regular_grid_size.x + infra_grid_size.x,
            y: regular_grid_size.y.max(infra_grid_size.y)
        };

        (total_size, infra_grid_size.x)
    }


    fn create_subnet_nodes(
        &self, 
        subnets: &[Subnet],
        layouts: &HashMap<Uuid, SubnetLayout>
    ) -> Vec<Node> {

        // Collect all subnet entries (including no_subnet) and sort by size
        let mut all_subnet_entries: Vec<(Uuid, &SubnetLayout)> = layouts.iter()
            .map(|(id, layout)| (*id, layout))
            .collect();
        
        // Sort by size (smallest first) - same logic as in create_subnet_child_nodes
        all_subnet_entries.sort_by_key(|(_, layout)| layout.size.x * layout.size.y);

        let container_grid_dimensions = self.calculate_container_grid_dimensions(all_subnet_entries.len());

        let (positions, _) = self.calculate_container_size_and_child_positions(
            all_subnet_entries.iter().map(|(id, layout)| (*id, layout.size.clone())).collect::<Vec<_>>(), 
            &container_grid_dimensions, 
            &SUBNET_PADDING
        );

        // Create all nodes (including no_subnet) from the sorted list
        all_subnet_entries.iter()
            .filter_map(|(subnet_id, layout)| {
                if let Some(position) = positions.get(subnet_id) {
                    let node_type = NodeType::SubnetNode;
                    
                    // Handle no_subnet case
                    if *subnet_id == self.no_subnet_id || *subnet_id == self.wan_id {
                        let label = if *subnet_id == self.no_subnet_id {"No Subnet"} else {"WAN"};
                        return Some(Node {
                            id: *subnet_id,
                            parent_id: None,
                            host_id: None,
                            interface_id: None,
                            node_type,
                            position: position.clone(),
                            size: layout.size.clone(),
                            infra_width: Some(layout.infra_width),
                            subnet_label: Some(label.to_string())
                        });
                    }
                    
                    // Handle regular subnet case
                    if subnets.iter().any(|s| s.id == *subnet_id) {
                        return Some(Node { 
                            id: *subnet_id,
                            parent_id: None,
                            host_id: None,
                            interface_id: None,
                            node_type,
                            position: position.clone(),
                            size: layout.size.clone(),
                            infra_width: Some(layout.infra_width),
                            subnet_label: None
                        });
                    }
                }
                None
            })
            .collect()
    }

    // 2nd pass
    fn add_interface_edges(&self, graph: &mut Graph<Node, Edge>, node_indices: &HashMap<Uuid, NodeIndex>, hosts: &[Host]) {

        hosts.iter().for_each(|host| {     
            if let Some(first_interface) = host.base.interfaces.first() {
                host.base.interfaces.iter()
                    .filter(|interface| interface.id != first_interface.id)
                    .for_each(|interface| {
                        if let (Some(&source_idx), Some(&target_idx)) = (node_indices.get(&interface.id), node_indices.get(&first_interface.id)) {
                            graph.add_edge(source_idx, target_idx, Edge {
                                edge_type: EdgeType::Interface,
                                source: first_interface.id,
                                target: interface.id,
                                source_handle: EdgeHandle::Top,
                                target_handle: EdgeHandle::Bottom
                            });
                        }
                    });
            }
        });

    }

    fn add_group_edges(
        &self, 
        graph: &mut Graph<Node, Edge>, 
        node_indices: &HashMap<Uuid, NodeIndex>, 
        groups: &[Group],
    ) {
        for host_group in groups {
            let bindings = &host_group.base.service_bindings;
            
            // Create sequential edges within each group (path-like connections)
            for window in bindings.windows(2) {
                if let (Some(&source_idx), Some(&target_idx)) = 
                    (node_indices.get(&window[0].interface_id), node_indices.get(&window[1].interface_id)) {
                    graph.add_edge(source_idx, target_idx, Edge {
                        edge_type: EdgeType::Group,
                        source: window[0].interface_id,
                        target: window[1].interface_id,
                        source_handle: EdgeHandle::Top,
                        target_handle: EdgeHandle::Bottom
                    });
                }
            }
        }
    }
}