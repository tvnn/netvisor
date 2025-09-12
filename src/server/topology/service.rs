use std::{collections::{HashMap, HashSet}, sync::Arc};

use anyhow::Error;
use petgraph::{graph::NodeIndex, Graph};
use uuid::Uuid;

use crate::server::{
    host_groups::{service::HostGroupService, types::HostGroup}, hosts::{service::HostService, types::base::Host}, services::types::types::ServiceType, shared::{types::metadata::TypeMetadataProvider}, subnets::{service::SubnetService, types::base::Subnet}, topology::types::base::{Edge, EdgeType, Node, NodeType, SubnetChild, SubnetLayout, XY}
};

const SUBNET_PADDING: XY = XY{x: 50, y: 50};

const NODE_SIZE: XY = XY{x: 100, y: 100};
const NODE_PADDING: XY = XY{x: 10, y: 10};

pub struct TopologyService {
    host_service: Arc<HostService>,
    subnet_service: Arc<SubnetService>,
    host_group_service: Arc<HostGroupService>,
    no_subnet_id: Uuid
}

impl TopologyService {
    pub fn new(host_service: Arc<HostService>, subnet_service: Arc<SubnetService>, host_group_service: Arc<HostGroupService>) -> Self {
        Self { 
            host_service,
            subnet_service,
            host_group_service,
            no_subnet_id: Uuid::new_v4()
        }
    }

    pub async fn build_graph(&self) -> Result<Graph<Node, Edge>, Error> {
        let hosts = self.host_service.get_all_hosts().await?;
        let subnets = self.subnet_service.get_all_subnets().await?;
        let host_groups = self.host_group_service.get_all_groups().await?;
                
        // First pass: create nodes with positions, determining layout from bottom up
        let (subnet_layouts, child_nodes) = self.create_subnet_child_nodes(&hosts, &subnets);
        let subnet_nodes = self.create_subnet_nodes(&subnets, &subnet_layouts);

        // Add nodes to graph
        let mut graph: Graph<Node, Edge> = Graph::new();
        let node_indices: HashMap<Uuid, NodeIndex> = subnet_nodes.into_iter().chain(child_nodes.into_iter()).map(|node| {
            let node_id = node.id;
            let node_idx = graph.add_node(node);
            (node_id, node_idx)
        })
        .collect();
        
        // Second pass: add edges
        self.add_subnet_infra_edges(&mut graph, &node_indices, &subnets);
        self.add_node_group_edges(&mut graph, &node_indices, &host_groups);
        self.add_interface_edges(&mut graph, &node_indices, &hosts);
        
        Ok(graph)
    }

    // Generic helper methods for positioning nodes in grids of arbitrary dimensions (rows / cols) and size (pixels)
    fn calculate_container_grid_dimensions(&self, children: usize) -> XY {
        if children == 0 {
            return XY { x: 1, y: 1 };
        }
        
        let x = (children as f64).sqrt().ceil() as usize;
        let y = ((children as f64) / x as f64).ceil() as usize;
        XY { x, y }
    }

    fn calculate_child_coordinates_in_grid(&self, grid: &XY, child_idx: usize) -> XY {
        XY {
            x: child_idx % grid.x,
            y: ((child_idx / grid.x) as f64).floor() as usize
        }
    }

    fn calculate_relative_position_in_container(&self, container_grid_dimensions: &XY, container_size: &XY, child_grid_position: &XY) -> XY {
        XY {
            x: ((child_grid_position.x as f64 / container_grid_dimensions.x as f64)*(container_size.x as f64)) as usize,
            y: ((child_grid_position.y as f64 / container_grid_dimensions.y as f64)*(container_size.y as f64)) as usize
        }
    }

    // 1st pass
    fn create_subnet_child_nodes(
        &self, 
        hosts: &[Host],
        subnets: &[Subnet],
    ) -> (HashMap<Uuid, SubnetLayout>, Vec<Node>){

        let children_by_subnet: HashMap<Uuid, Vec<SubnetChild>> = hosts.iter()
            .flat_map(|host| {
                let primary_id = host.primary_interface().map(|p| p.id);
                
                // Create host node
                let host_node = match host.primary_interface() {
                    Some(primary) => {
                        let subnet_child = SubnetChild {
                            host: host.clone(),
                            interface: Some(primary.clone()),
                            node_type: NodeType::HostNode,
                            id: host.id,
                            label: host.base.name.clone()
                        };
                        (primary.base.subnet_id, subnet_child)
                    },
                    None => {
                        let subnet_child = SubnetChild {
                            host: host.clone(),
                            interface: None,
                            node_type: NodeType::HostNode,
                            id: host.id,
                            label: host.base.name.clone()
                        };
                        (self.no_subnet_id, subnet_child)
                    }
                };
                
                // Create interface nodes for non-primary interfaces
                let mut interface_nodes: Vec<(Uuid, SubnetChild)> = host.base.interfaces.iter()
                    .filter(move |interface| Some(interface.id) != primary_id)
                    .map(|interface| {
                        let subnet_child = SubnetChild {
                            host: host.clone(),
                            interface: Some(interface.clone()),
                            node_type: NodeType::InterfaceNode,
                            id: interface.id,
                            label: interface.base.name.clone()
                        };
                        (interface.base.subnet_id, subnet_child)
                    })
                    .collect();
                
                // Chain the host node with interface nodes
                interface_nodes.push(host_node);
                interface_nodes
            })
            .fold(HashMap::new(), |mut acc, (subnet_id, child)| {
                acc.entry(subnet_id).or_insert_with(Vec::new).push(child);
                acc
            });

        let mut child_nodes = Vec::new();

        let subnet_layouts: HashMap<Uuid, SubnetLayout> = children_by_subnet.iter().map(|(subnet_id, children)| {

            if let Some(subnet) = subnets.iter().find(|s| s.id == *subnet_id) {
                // let gateways: Vec<&SubnetChild> = children.iter().filter(|c| c.node_type == NodeType::HostNode && subnet.base.gateways.contains(&c.host.id)).collect();
                // let dns: Vec<&SubnetChild> = children.iter().filter(|c| c.node_type == NodeType::HostNode && subnet.base.dns_resolvers.contains(&c.host.id)).collect();
                // let reverse_proxies: Vec<&SubnetChild> = children.iter().filter(|c| c.node_type == NodeType::HostNode && subnet.base.reverse_proxies.contains(&c.host.id)).collect();

                // let special_host_ids: HashSet<_> = subnet.base.gateways.iter()
                //     .chain(&subnet.base.dns_resolvers)
                //     .chain(&subnet.base.reverse_proxies)
                //     .collect();

                // let other_children: Vec<&SubnetChild> = children.iter().filter(|c| c.node_type == NodeType::HostNode && !special_host_ids.contains(&c.host.id)).collect();
                
                // let other_children_grid_dimensions = self.calculate_container_grid_dimensions(other_children.len());
                // let other_children_grid_size = XY{
                //     x: other_children_grid_dimensions.x*(NODE_SIZE.x + NODE_PADDING.x),
                //     y: other_children_grid_dimensions.y*(NODE_SIZE.y + NODE_PADDING.y)
                // };
            };

                let all_children_grid_dimensions = self.calculate_container_grid_dimensions(children.len());
                let all_children_size = XY {
                    x: all_children_grid_dimensions.x*(NODE_SIZE.x + NODE_PADDING.x),
                    y: all_children_grid_dimensions.y*(NODE_SIZE.y + NODE_PADDING.y)
                };

                println!("Subnet {:?} has {} children", subnet_id, children.len());
                println!("Grid dimensions: {:?}", all_children_grid_dimensions);
                println!("Grid size: {:?}", all_children_size);
                
                children.iter().enumerate().for_each(|(idx, child)| {
                    let grid_position = self.calculate_child_coordinates_in_grid(&all_children_grid_dimensions, idx);
                    let relative_position = self.calculate_relative_position_in_container(&all_children_grid_dimensions, &all_children_size, &grid_position);

                    let default_service_type = match child.host.default_service() {
                        Some(service) => &service.base.service_type,
                        None => &ServiceType::Unknown
                    };

                    let (label, id, node_type) = (child.label.clone(), child.id, child.node_type.clone());

                    child_nodes.push(Node { 
                        color: default_service_type.color().to_string(),
                        icon: default_service_type.icon().to_string(),
                        id,
                        node_type,
                        label,
                        parent_id: Some(*subnet_id), 
                        position: XY {
                            x: relative_position.x + NODE_PADDING.x,
                            y: relative_position.y + NODE_PADDING.y
                        },
                        size: NODE_SIZE
                    });
                });

                return (*subnet_id, SubnetLayout {
                    size: XY {
                        x: all_children_size.x + NODE_PADDING.x,
                        y: all_children_size.y + NODE_PADDING.y
                    },
                    grid_dimensions: all_children_grid_dimensions
                });
        })
        .collect();

        (subnet_layouts, child_nodes)
    }

    fn create_subnet_nodes(
        &self, 
        subnets: &[Subnet],
        layouts: &HashMap<Uuid, SubnetLayout>
    ) -> Vec<Node> {

       let subnet_positions = self.calculate_subnet_positions(layouts, subnets);
        
        subnets.iter().filter_map(|subnet| {
            if let (Some(position), Some(layout)) = (subnet_positions.get(&subnet.id), layouts.get(&subnet.id)) {
                return Some(Node { 
                    color: subnet.base.subnet_type.color().to_string(),
                    icon: subnet.base.subnet_type.icon().to_string(),
                    id: subnet.id,
                    label: subnet.base.name.clone(),
                    parent_id: None,
                    node_type: NodeType::SubnetNode,
                    position: position.clone(),
                    size: layout.size.clone()
                })
            }
            None
        })
        .collect()
    }

    fn calculate_subnet_positions(&self, layouts: &HashMap<Uuid, SubnetLayout>, subnets: &[Subnet]) -> HashMap<Uuid, XY> {
        let mut positions = HashMap::new();
        let subnet_count = subnets.len() + if layouts.contains_key(&self.no_subnet_id) {1} else {0};
        let container_grid_dimensions = self.calculate_container_grid_dimensions(subnet_count);
        
        // Group subnets by their grid row
        let mut rows: Vec<Vec<(Uuid, &SubnetLayout)>> = vec![Vec::new(); container_grid_dimensions.y];

        // Handle no-subnet case (always at index 0)
        if let Some(no_subnet_layout) = layouts.get(&self.no_subnet_id) {
            let grid_pos = self.calculate_child_coordinates_in_grid(&container_grid_dimensions, 0);
            rows[grid_pos.y].push((self.no_subnet_id, no_subnet_layout));
        }
        
        for (idx, subnet) in subnets.iter().enumerate() {
            if let Some(layout) = layouts.get(&subnet.id) {
                let grid_pos = self.calculate_child_coordinates_in_grid(&container_grid_dimensions, idx + 1);
                rows[grid_pos.y].push((subnet.id, layout));
            }
        }
        
        let mut current_y = 0;
        
        for row in rows {
            if row.is_empty() {
                continue;
            }
            
            let mut current_x = 0;
            let mut max_height_in_row = 0;
            
            for (subnet_id, layout) in row {
                positions.insert(subnet_id, XY { x: current_x, y: current_y });
                current_x += layout.size.x + SUBNET_PADDING.x;
                max_height_in_row = max_height_in_row.max(layout.size.y);
            }
            
            current_y += max_height_in_row + SUBNET_PADDING.y;
        }
        
        positions
    }

    // 2nd pass
    fn add_subnet_infra_edges(&self, graph: &mut Graph<Node, Edge>, node_indices: &HashMap<Uuid, NodeIndex>, subnets: &[Subnet]) {
        for subnet in subnets {
            // Gateway edges: Connect gateways to all nodes in their subnet
            for gateway_id in &subnet.base.gateways {
                if let Some(&gateway_idx) = node_indices.get(gateway_id) {
                    for host_id in &subnet.base.hosts {
                        if let Some(&host_idx) = node_indices.get(host_id) {
                            if gateway_id != host_id {
                                graph.add_edge(gateway_idx, host_idx, Edge {
                                    edge_type: EdgeType::Gateway,
                                    source: *host_id,
                                    target: *gateway_id,
                                });
                            }
                        }
                    }
                }
            }
            
            // DNS edges: Connect DNS servers to nodes they serve
            for dns_id in &subnet.base.dns_resolvers {
                if let Some(&dns_idx) = node_indices.get(dns_id) {
                    for host_id in &subnet.base.hosts {
                        if let Some(&host_idx) = node_indices.get(host_id) {
                            if dns_id != host_id {
                                graph.add_edge(dns_idx, host_idx, Edge {
                                    edge_type: EdgeType::DnsResolution,
                                    source: *host_id,
                                    target: *dns_id,
                                });
                            }
                        }
                    }
                }
            }

            // Reverse proxy edges
            for rproxy_id in &subnet.base.reverse_proxies {
                if let Some(&rproxy_idx) = node_indices.get(rproxy_id) {
                    for host_id in &subnet.base.hosts {
                        if let Some(&host_idx) = node_indices.get(host_id) {
                            if rproxy_id != host_id {
                                graph.add_edge(rproxy_idx, host_idx, Edge {
                                    edge_type: EdgeType::ReverseProxy,
                                    source: *host_id,
                                    target: *rproxy_id,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_interface_edges(&self, graph: &mut Graph<Node, Edge>, node_indices: &HashMap<Uuid, NodeIndex>, hosts: &[Host]) {

        hosts.iter().for_each(|host| {

            let primary_id = match host.primary_interface() {
                Some(interface) => interface.id,
                None => Uuid::nil()
            };
            
            host.base.interfaces.iter()
                .filter(|interface| interface.id != primary_id)
                .for_each(|interface| {
                    if let (Some(&source_idx), Some(&target_idx)) = (node_indices.get(&interface.id), node_indices.get(&host.id)) {
                        graph.add_edge(source_idx, target_idx, Edge {
                            edge_type: EdgeType::HostInterface,
                            source: host.id,
                            target: interface.id
                        });
                    }
                });
        });

    }

    fn add_node_group_edges(
        &self, 
        graph: &mut Graph<Node, Edge>, 
        node_indices: &HashMap<Uuid, NodeIndex>, 
        host_groups: &[HostGroup]
    ) {
        for host_group in host_groups {
            let group_host_ids = &host_group.base.hosts;
            
            // Create sequential edges within each group (path-like connections)
            for window in group_host_ids.windows(2) {
                if let (Some(&source_idx), Some(&target_idx)) = 
                    (node_indices.get(&window[0]), node_indices.get(&window[1])) {
                    graph.add_edge(source_idx, target_idx, Edge {
                        edge_type: EdgeType::HostGroup,
                        source: window[0],
                        target: window[1]
                    });
                }
            }
        }
    }
}