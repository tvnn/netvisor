use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use petgraph::{graph::NodeIndex, Graph};
use uuid::Uuid;

use crate::server::{
    host_groups::{service::HostGroupService, types::HostGroup}, hosts::{service::HostService, types::base::Host}, services::types::types::ServiceType, shared::types::metadata::TypeMetadataProvider, subnets::{service::SubnetService, types::base::Subnet}, topology::types::base::{Edge, EdgeType, Node, NodeType, SubnetLayout, XY}
};

const CONTAINER_SIZE: usize = 1000;
const GRID_SCALE: usize = 100;
const MIN_SUBNET_SIZE: XY = XY{x: GRID_SCALE, y: GRID_SCALE};
const HOST_SIZE: XY = XY{x: GRID_SCALE, y: GRID_SCALE};

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

        let mut graph = Graph::new();
        let mut node_indices = HashMap::new();
        
        // First pass: analyze subnet composition and calculate layouts
        let subnet_layouts = self.calculate_subnet_layouts(&subnets, &hosts);
        
        // Second pass: create nodes with positions
        self.create_subnet_nodes(&mut graph, &mut node_indices, &subnets, &subnet_layouts);
        self.create_host_nodes(&mut graph, &mut node_indices, &hosts, &subnet_layouts);
        
        // Third pass: add edges
        self.add_subnet_infra_edges(&mut graph, &node_indices, &subnets);
        self.add_node_group_edges(&mut graph, &node_indices, &host_groups);
        
        Ok(graph)
    }

    fn calculate_subnet_layouts(&self, subnets: &[Subnet], hosts: &[Host]) -> HashMap<Uuid, SubnetLayout> {
        let mut layouts = HashMap::new();
        let subnet_count = subnets.len();
        let container_size = XY {
            x: CONTAINER_SIZE,
            y: CONTAINER_SIZE
        };
        let container_grid = self.calculate_container_grid_dimensions(subnet_count+1); // +1 for hosts w/o a subnet
        
        let hosts_without_subnet_count = hosts.iter().filter(|h| h.base.interfaces.len() == 0).count();
        let (subnet_size, subnet_grid_dimensions) = self.calculate_subnet_size_and_dimensions(hosts_without_subnet_count);
        let grid_pos = self.calculate_child_coordinates_in_grid(&container_grid, 0);
        let relative_position = self.calculate_relative_position_in_container(&container_grid, &container_size, &grid_pos);

        for (subnet_idx, subnet) in subnets.iter().enumerate() {
            let host_count = subnet.base.hosts.len();
            let (subnet_size, subnet_grid_dimensions) = self.calculate_subnet_size_and_dimensions(host_count);
            
            // Calculate grid position in container for this subnet
            let grid_pos = self.calculate_child_coordinates_in_grid(&container_grid, subnet_idx+1);
            let relative_position = self.calculate_relative_position_in_container(&container_grid, &container_size, &grid_pos);
            
            tracing::info!("Subnet {} position {:?} relative_position {:?}", subnet.base.name, grid_pos, relative_position);

            layouts.insert(subnet.id, SubnetLayout {
                position: relative_position,
                size: subnet_size,
                grid_dimensions: subnet_grid_dimensions
            });
        }

        tracing::info!("No subnet position {:?} relative_position {:?}", grid_pos, relative_position);

        layouts.insert(self.no_subnet_id, SubnetLayout { position: relative_position, size: subnet_size, grid_dimensions: subnet_grid_dimensions });
        
        layouts
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



    fn calculate_subnet_size_and_dimensions(&self, host_count: usize) -> (XY, XY) {
        if host_count == 0 {
            return (
                MIN_SUBNET_SIZE,
                XY { x: 1, y: 1}
            )
        }

        // Calculate grid needed for hosts
        let dimensions = self.calculate_container_grid_dimensions(host_count);
                
        let needed_width = dimensions.x * HOST_SIZE.x;
        let needed_height = dimensions.y * HOST_SIZE.y;

        tracing::info!("Needed width {} height {}, dimensions {:?}", needed_width, needed_height, dimensions);
        
        // Use calculated size or minimum, whichever is larger
        (
            XY {
                x: needed_width.max(MIN_SUBNET_SIZE.x),
                y: needed_height.max(MIN_SUBNET_SIZE.y),
            },
            dimensions
        )
    }

    fn create_subnet_nodes(
        &self, 
        graph: &mut Graph<Node, Edge>, 
        node_indices: &mut HashMap<Uuid, NodeIndex>,
        subnets: &[Subnet],
        layouts: &HashMap<Uuid, SubnetLayout>
    ) {
        for subnet in subnets {
            let layout = layouts.get(&subnet.id).unwrap();
            
            let subnet_node = Node { 
                color: subnet.base.subnet_type.color().to_string(),
                icon: subnet.base.subnet_type.icon().to_string(),
                id: subnet.id,
                label: subnet.base.name.clone(),
                parent_id: None,
                node_type: NodeType::SubnetNode,
                position: layout.position.clone(),
                size: layout.size.clone()
            };
            
            let node_index = graph.add_node(subnet_node);
            node_indices.insert(subnet.id, node_index);
        }
    }

    fn create_host_nodes(
        &self, 
        graph: &mut Graph<Node, Edge>, 
        node_indices: &mut HashMap<Uuid, NodeIndex>,
        hosts: &[Host],
        layouts: &HashMap<Uuid, SubnetLayout>
    ) {
        let mut hosts_by_subnet: HashMap<Uuid, Vec<&Host>> = HashMap::new();
        for host in hosts {
            let subnet_id = match host.primary_interface() {
                Some(interface) => interface.base.subnet_id,
                None => self.no_subnet_id
            };

            hosts_by_subnet.entry(subnet_id).or_default().push(host);
        }

        hosts_by_subnet.iter().for_each(|(subnet_id, hosts)| {
            
            if let Some(layout) = layouts.get(subnet_id) {

                tracing::info!("Subnet {} relative_position {:?} size {:?}", subnet_id, layout.position, layout.size);

                hosts.iter().enumerate().for_each(|(idx, host)| {

                    let grid_position = self.calculate_child_coordinates_in_grid(&layout.grid_dimensions, idx);
                    let relative_position: XY = self.calculate_relative_position_in_container(&layout.grid_dimensions, &layout.size, &grid_position);
                    
                    tracing::info!("Host {} grid position {:?} relative position {:?}", host.base.name, grid_position, relative_position);

                    let default_service_type = match host.default_service() {
                        Some(service) => &service.base.service_type,
                        None => &ServiceType::Unknown
                    };

                    let graph_node = Node { 
                        color: default_service_type.color().to_string(),
                        icon: default_service_type.icon().to_string(),
                        id: host.id,
                        label: host.base.name.clone(),
                        parent_id: Some(*subnet_id), 
                        node_type: NodeType::HostNode,
                        position: relative_position,
                        size: HOST_SIZE
                    };

                    let node_index = graph.add_node(graph_node);
                    node_indices.insert(host.id, node_index);
                });
            }
        });
    }


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