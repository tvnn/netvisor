use std::sync::Arc;

use anyhow::Error;

use crate::server::{host_groups::service::HostGroupService, hosts::service::HostService, subnets::service::SubnetService};

pub struct TopologyService {
    host_service: Arc<HostService>,
    subnet_service: Arc<SubnetService>,
    host_group_service: Arc<HostGroupService>,
}

impl TopologyService {
    pub fn new(host_service: Arc<HostService>, subnet_service: Arc<SubnetService>, host_group_service: Arc<HostGroupService>) -> Self {
        Self { 
            host_service,
            subnet_service,
            host_group_service,
        }
    }

    pub async fn generate_topology_graph(&self) -> Result<String, Error> {
        let hosts= self.host_service.get_all_hosts().await?;
        let subnets = self.subnet_service.get_all_subnets().await?;
        let host_groups= self.host_group_service.get_all_groups().await?;

        // let graph = petgraph::Graph::new();
        Ok("".to_string())
    }

    // pub async fn build_graph(&self, hosts: Vec<Host>, subnets, host_groups) {
    //     let mut graph = petgraph::Graph::new();
    //     let mut node_indices = HashMap::new();

    //     hosts.iter().for_each
    // }
}