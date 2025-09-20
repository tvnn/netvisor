use std::{collections::{HashMap, HashSet}, sync::Arc};

use anyhow::Error;
use petgraph::{graph::NodeIndex, Graph};
use uuid::Uuid;

use crate::server::{
    host_groups::{service::HostGroupService, types::HostGroup}, hosts::{service::HostService, types::base::Host}, services::{service::ServiceService, types::{base::Service, types::ServiceType}}, shared::types::metadata::TypeMetadataProvider, subnets::{service::SubnetService, types::base::{Subnet, SubnetBase}}, topology::types::base::{Edge, EdgeType, Node, NodeLayout, NodeType, SubnetChild, SubnetChildNodeSize, XY}
};

const SUBNET_PADDING: XY = XY{x: 50, y: 50};
const NODE_PADDING: XY = XY{x: 25, y: 25};

pub struct TopologyService {
    host_service: Arc<HostService>,
    subnet_service: Arc<SubnetService>,
    host_group_service: Arc<HostGroupService>,
    service_service: Arc<ServiceService>,
    no_subnet_id: Uuid,
}

impl TopologyService {
    pub fn new(host_service: Arc<HostService>, subnet_service: Arc<SubnetService>, host_group_service: Arc<HostGroupService>, service_service: Arc<ServiceService>) -> Self {
        Self { 
            host_service,
            subnet_service,
            host_group_service,
            service_service,
            no_subnet_id: Uuid::nil()
        }
    }

    pub async fn build_graph(&self) -> Result<Graph<Node, Edge>, Error> {
        let hosts = self.host_service.get_all_hosts().await?;
        let subnets = self.subnet_service.get_all_subnets().await?;
        let host_groups = self.host_group_service.get_all_groups().await?;
        let services = self.service_service.get_all_services().await?;
                
        // First pass: create nodes with positions, determining layout from bottom up
        let (subnet_sizes, child_nodes) = self.create_subnet_child_nodes(&hosts, &subnets, &services);
        let subnet_nodes = self.create_subnet_nodes(&subnets, &subnet_sizes);

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
    fn calculate_container_size_and_child_positions(&self, child_sizes: HashMap<Uuid, XY>, container_grid: &XY, padding: &XY) -> (HashMap<Uuid, XY>, XY) {
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
            max_y = max_y.max(current_y + max_height_in_row);
        }

        let container_size = XY {
            x: max_x + padding.x,
            y: max_y + padding.y
        };

        (child_positions, container_size)
    }

    // 1st pass
    fn create_subnet_child_nodes(
        &self, 
        hosts: &[Host],
        subnets: &[Subnet],
        services: &[Service]
    ) -> (HashMap<Uuid, XY>, Vec<Node>) {
        let children_by_subnet = self.group_children_by_subnet(hosts, services);
        let mut child_nodes = Vec::new();

        let subnet_sizes: HashMap<Uuid, XY> = children_by_subnet.iter().map(|(subnet_id, children)| {
            let size = self.calculate_subnet_size(*subnet_id, children, subnets, services, &mut child_nodes);
            (*subnet_id, size)
        }).collect();

        (subnet_sizes, child_nodes)
    }

    fn group_children_by_subnet(&self, hosts: &[Host], services: &[Service]) -> HashMap<Uuid, Vec<SubnetChild>> {
        hosts.iter()
            .flat_map(|host| {

                let services_for_host: Vec<Service> = services.iter().filter(|s| s.base.host_id == host.id).cloned().collect();

                // Create primary node for host
                let primary_service: Option<&Service> = services_for_host.iter()
                    .filter(|s| !s.base.service_type.is_generic_service())
                    .max_by_key(|s| s.base.interface_bindings.len())
                    .or_else(||services_for_host.first());
                
                let primary_service_interface = primary_service
                    .as_ref()
                    .and_then(|p| p.base.interface_bindings.first())
                    .and_then(|binding| host.get_interface(binding));

                let (subnet_id, primary_service_interface_id) = match primary_service_interface {
                    Some(interface) => (interface.base.subnet_id, interface.id),
                    None => (self.no_subnet_id, Uuid::nil())
                };

                let (services_on_primary_interface, other_services): (Vec<_>, Vec<_>) = services_for_host
                    .iter()
                    .partition(|s| {
                        s.base.interface_bindings.iter().any(|binding_id| {
                            host.get_interface(binding_id)
                                .map_or(false, |interface| interface.id == primary_service_interface_id)
                        })
                    });

                let services_on_primary_interface_ids: Vec<Uuid> = services_on_primary_interface.iter().map(|s| s.id).collect();

                let host_node = (subnet_id, SubnetChild {
                    host: host.clone(),
                    interface: primary_service_interface.cloned(),
                    node_type: NodeType::HostNode,
                    id: host.id,
                    label: Some(host.base.name.clone()),
                    size: SubnetChildNodeSize::from_service_count(services_for_host.len()),
                    services: services_on_primary_interface_ids
                });
                
                // Create interface nodes for non-primary interfaces
                let mut interface_nodes: Vec<(Uuid, SubnetChild)> = host.base.interfaces.iter()
                    .filter(|interface| interface.id != primary_service_interface_id)
                    .map(|interface| {

                        let services: Vec<Uuid> = other_services.iter()
                            .filter_map(|s| if s.base.interface_bindings.contains(&interface.id) {Some(s.id)} else {None})
                            .collect();

                        let subnet_child = SubnetChild {
                            host: host.clone(),
                            interface: Some(interface.clone()),
                            node_type: NodeType::InterfaceNode,
                            id: interface.id,
                            label: interface.base.name.clone(),
                            size: SubnetChildNodeSize::from_service_count(services.len()),
                            services
                        };
                        (interface.base.subnet_id, subnet_child)
                    })
                    .collect();
                
                interface_nodes.push(host_node);
                interface_nodes
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
    ) -> XY {
        // Separate infrastructure from regular nodes
        let (infrastructure_children, regular_children) = if let Some(subnet) = subnets.iter().find(|s| s.id == subnet_id) {
            let infrastructure_host_ids: Vec<Uuid> = subnet.base.gateways.iter()
                .chain(&subnet.base.dns_resolvers)
                .chain(&subnet.base.reverse_proxies)
                .filter_map(|id| services.iter().find(|s| s.id == *id))
                .map(|s| s.base.host_id)
                .collect();

            let infrastructure: Vec<_> = children.iter().filter(|c| 
                c.node_type == NodeType::HostNode && infrastructure_host_ids.contains(&c.host.id)
            ).collect();
            
            let regular: Vec<_> = children.iter().filter(|c| 
                c.node_type != NodeType::HostNode || !infrastructure_host_ids.contains(&c.host.id)
            ).collect();
            
            (infrastructure, regular)
        } else {
            (Vec::new(), children.iter().collect())
        };

        // Calculate regular nodes layout
        let (regular_grid_size, regular_grid_dimensions, regular_child_positions) = {
            let regular_grid_dimensions = self.calculate_container_grid_dimensions(regular_children.len());
            let (regular_child_positions, regular_grid_size) = self.calculate_container_size_and_child_positions(
                regular_children.iter().map(|c| (c.id, c.size.size())).collect(),
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
                infrastructure_children.iter().map(|c| (c.id, c.size.size())).collect(),
                &infra_grid_dimensions,
                &NODE_PADDING
            );
            (infra_grid_size, infra_child_positions, infra_cols)
        };
        
        infrastructure_children.iter().for_each(|child| {
            if let Some(position) = infra_child_positions.get(&child.id) {
                child_nodes.push(Node { 
                    id: child.id,
                    node_type: child.node_type.clone(),
                    label: child.label.clone(),
                    parent_id: Some(subnet_id), 
                    position: position.clone(),
                    size: child.size.size()
                });
            }
        });

        regular_children.iter().for_each(|child| {
            if let Some(position) = regular_child_positions.get(&child.id) {
                // Shift position to the right if there are infra columns
                let node_position = XY {
                    x: position.x + if infra_cols > 0 {infra_grid_size.x + NODE_PADDING.x} else {0},
                    y: position.y
                };
                child_nodes.push(Node { 
                    id: child.id,
                    node_type: child.node_type.clone(),
                    label: child.label.clone(),
                    parent_id: Some(subnet_id), 
                    position: node_position,
                    size: child.size.size()
                });
            };
        });

        let total_size = XY {
            x: regular_grid_size.x + infra_grid_size.x + if infra_grid_size.x > 0 {NODE_PADDING.x} else {0},
            y: regular_grid_size.y.max(infra_grid_size.y)
        };

        // let total_grid_dimensions = XY {
        //     x: regular_grid_dimensions.x + infra_grid_dimensions.x,
        //     y: regular_grid_dimensions.y.max(infra_grid_dimensions.y)
        // };

        total_size
    }


    fn create_subnet_nodes(
        &self, 
        subnets: &[Subnet],
        sizes: &HashMap<Uuid, XY>
    ) -> Vec<Node> {

        let container_grid_dimensions = self.calculate_container_grid_dimensions(subnets.len()+1);

        let (positions, _) = self.calculate_container_size_and_child_positions(sizes.clone(), &container_grid_dimensions, &SUBNET_PADDING);

        let no_subnet_node = match (sizes.get(&self.no_subnet_id), positions.get(&self.no_subnet_id)) {
            (Some(size), Some(position)) => {
                Some(Node {
                    id: self.no_subnet_id,
                    label: None,
                    parent_id: None,
                    node_type: NodeType::SubnetNode,
                    position: position.clone(),
                    size: size.clone()
                })
            },
            _ => None
        };

        subnets.iter()
            .filter_map(|subnet| {
                if let (Some(position), Some(size)) = (positions.get(&subnet.id), sizes.get(&subnet.id)) {
                    return Some(Node { 
                        id: subnet.id,
                        label: Some(subnet.base.cidr.to_string()),
                        parent_id: None,
                        node_type: NodeType::SubnetNode,
                        position: position.clone(),
                        size: size.clone()
                    })
                }
                None
            })
            .chain(no_subnet_node.into_iter())
            .collect()
    }

    fn calculate_subnet_positions(&self, layouts: &HashMap<Uuid, NodeLayout>, subnets: &[Subnet]) -> HashMap<Uuid, XY> {
        let mut positions = HashMap::new();
        let subnet_count = subnets.len() + if layouts.contains_key(&self.no_subnet_id) {1} else {0};
        let container_grid_dimensions = self.calculate_container_grid_dimensions(subnet_count);
        
        // Group subnets by their grid row
        let mut rows: Vec<Vec<(Uuid, &NodeLayout)>> = vec![Vec::new(); container_grid_dimensions.y];

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
        // for subnet in subnets {
        //     // Gateway edges: Connect gateways to all nodes in their subnet
        //     for gateway_id in &subnet.base.gateways {
        //         if let Some(&gateway_idx) = node_indices.get(gateway_id) {
        //             for host_id in &subnet.base.hosts {
        //                 if let Some(&host_idx) = node_indices.get(host_id) {
        //                     if gateway_id != host_id {
        //                         graph.add_edge(gateway_idx, host_idx, Edge {
        //                             edge_type: EdgeType::Gateway,
        //                             source: *host_id,
        //                             target: *gateway_id,
        //                         });
        //                     }
        //                 }
        //             }
        //         }
        //     }
            
        //     // DNS edges: Connect DNS servers to nodes they serve
        //     for dns_id in &subnet.base.dns_resolvers {
        //         if let Some(&dns_idx) = node_indices.get(dns_id) {
        //             for host_id in &subnet.base.hosts {
        //                 if let Some(&host_idx) = node_indices.get(host_id) {
        //                     if dns_id != host_id {
        //                         graph.add_edge(dns_idx, host_idx, Edge {
        //                             edge_type: EdgeType::DnsResolution,
        //                             source: *host_id,
        //                             target: *dns_id,
        //                         });
        //                     }
        //                 }
        //             }
        //         }
        //     }

        //     // Reverse proxy edges
        //     for rproxy_id in &subnet.base.reverse_proxies {
        //         if let Some(&rproxy_idx) = node_indices.get(rproxy_id) {
        //             for host_id in &subnet.base.hosts {
        //                 if let Some(&host_idx) = node_indices.get(host_id) {
        //                     if rproxy_id != host_id {
        //                         graph.add_edge(rproxy_idx, host_idx, Edge {
        //                             edge_type: EdgeType::ReverseProxy,
        //                             source: *host_id,
        //                             target: *rproxy_id,
        //                         });
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn add_interface_edges(&self, graph: &mut Graph<Node, Edge>, node_indices: &HashMap<Uuid, NodeIndex>, hosts: &[Host]) {

        // hosts.iter().for_each(|host| {

        //     let primary_id = match host.primary_interface() {
        //         Some(interface) => interface.id,
        //         None => Uuid::nil()
        //     };
            
        //     host.base.interfaces.iter()
        //         .filter(|interface| interface.id != primary_id)
        //         .for_each(|interface| {
        //             if let (Some(&source_idx), Some(&target_idx)) = (node_indices.get(&interface.id), node_indices.get(&host.id)) {
        //                 graph.add_edge(source_idx, target_idx, Edge {
        //                     edge_type: EdgeType::HostInterface,
        //                     source: host.id,
        //                     target: interface.id
        //                 });
        //             }
        //         });
        // });

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