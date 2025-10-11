use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use petgraph::{graph::NodeIndex, Graph};
use uuid::Uuid;

use crate::server::{
    groups::service::GroupService,
    hosts::service::HostService,
    services::service::ServiceService,
    subnets::service::SubnetService,
    topology::{
        service::{
            context::TopologyContext, edge_builder::EdgeBuilder, optimizer::TopologyOptimizer,
            subnet_layout_planner::SubnetLayoutPlanner,
        },
        types::{edges::Edge, nodes::Node},
    },
};

pub struct TopologyService {
    host_service: Arc<HostService>,
    subnet_service: Arc<SubnetService>,
    group_service: Arc<GroupService>,
    service_service: Arc<ServiceService>,
}

impl TopologyService {
    pub fn new(
        host_service: Arc<HostService>,
        subnet_service: Arc<SubnetService>,
        group_service: Arc<GroupService>,
        service_service: Arc<ServiceService>,
    ) -> Self {
        Self {
            host_service,
            subnet_service,
            group_service,
            service_service,
        }
    }

    pub async fn build_graph(&self) -> Result<Graph<Node, Edge>, Error> {
        // Fetch all data
        let hosts = self.host_service.get_all_hosts().await?;
        let subnets = self.subnet_service.get_all_subnets().await?;
        let groups = self.group_service.get_all_groups().await?;
        let services = self.service_service.get_all_services().await?;

        // Create context to avoid parameter passing
        let ctx = TopologyContext::new(&hosts, &subnets, &services, &groups);

        // Create all edges (needed for anchor analysis)
        let interface_edges = EdgeBuilder::create_interface_edges(&ctx);
        let group_edges = EdgeBuilder::create_group_edges(&ctx);
        let all_edges: Vec<Edge> = interface_edges.into_iter().chain(group_edges).collect();

        // Create nodes with layout
        let mut layout_planner = SubnetLayoutPlanner::new();
        let (subnet_layouts, child_nodes) =
            layout_planner.create_subnet_child_nodes(&ctx, &all_edges);

        // Get relocation info from layout planner
        let relocation_map = layout_planner.get_handle_relocation_map();

        let subnet_nodes = layout_planner.create_subnet_nodes(&ctx, &subnet_layouts);

        // Optimize node positions and handle edge adjustments
        let optimizer = TopologyOptimizer::new();
        let mut all_nodes: Vec<Node> = subnet_nodes.into_iter().chain(child_nodes).collect();
        let all_edges = optimizer.optimize_graph(&mut all_nodes, all_edges, relocation_map);

        // Build graph
        let mut graph: Graph<Node, Edge> = Graph::new();
        let node_indices: HashMap<Uuid, NodeIndex> = all_nodes
            .into_iter()
            .map(|node| {
                let node_id = node.id;
                let node_idx = graph.add_node(node);
                (node_id, node_idx)
            })
            .collect();

        // Add edges to graph
        EdgeBuilder::add_edges_to_graph(&mut graph, &node_indices, all_edges);

        Ok(graph)
    }
}
