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
        service::{edges::TopologyEdgePlanner, nodes::TopologyNodePlanner},
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
        let hosts = self.host_service.get_all_hosts().await?;
        let subnets = self.subnet_service.get_all_subnets().await?;
        let groups = self.group_service.get_all_groups().await?;
        let services = self.service_service.get_all_services().await?;

        let node_planner = TopologyNodePlanner::new();
        let edge_planner = TopologyEdgePlanner::new();

        // First pass: create nodes with positions, determining layout from bottom up
        let (subnet_sizes, child_nodes) =
            node_planner.create_subnet_child_nodes(&hosts, &subnets, &services, &groups);
        let subnet_nodes = node_planner.create_subnet_nodes(&subnets, &subnet_sizes);

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
        edge_planner.add_group_edges(&mut graph, &node_indices, &groups, &hosts, &subnets);
        edge_planner.add_interface_edges(&mut graph, &node_indices, &hosts, &subnets);

        Ok(graph)
    }
}
