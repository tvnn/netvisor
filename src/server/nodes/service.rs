use anyhow::{Error, Result};
use futures::future::try_join_all;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    node_groups::storage::NodeGroupStorage, nodes::{
        storage::NodeStorage,
        types::{
            api::{NodeSubnetRelationshipChange, NodeUpdateRequest}, base::{Node, NodeBase}
        }
    }, subnets::{service::SubnetService, types::base::{Subnet}}, utils::base::{NetworkUtils, ServerNetworkUtils}
};

pub struct NodeService {
    storage: Arc<dyn NodeStorage>,
    group_storage: Arc<dyn NodeGroupStorage>,
    subnet_service: Arc<SubnetService>,
    utils: ServerNetworkUtils
}

impl NodeService {
    pub fn new(storage: Arc<dyn NodeStorage>, group_storage: Arc<dyn NodeGroupStorage>, subnet_service: Arc<SubnetService>, utils: ServerNetworkUtils) -> Self {
        Self { 
            storage,
            group_storage,
            subnet_service,
            utils
        }
    }

    /// Create a new node
    pub async fn create_node(&self, node_base: NodeBase) -> Result<Node> {
        
        let node = Node::new(node_base);

        let all_nodes = self.storage.get_all().await?;

        let node_from_storage = match all_nodes.iter().find(|n| node.eq(n)) {
            Some(existing_node) => {
                existing_node.clone()
            }
            None => {
                let node = self.update_default_subnet_membership(node).await?;
                self.storage.create(&node).await?;
                self.update_subnet_node_relationships(&node).await?;
                node
            }
        };

        Ok(node_from_storage)
    }

    pub async fn get_node(&self, id: &Uuid) -> Result<Option<Node>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_nodes(&self) -> Result<Vec<Node>> {
        self.storage.get_all().await
    }

    pub async fn update_node(&self, id: &Uuid, updates: NodeUpdateRequest) -> Result<(Node, NodeSubnetRelationshipChange), Error> {
        
        let mut node = match self.get_node(&id).await? {
            Some(n) => n,
            None => {
                let msg = format!("Node '{}' not found", id);
                return Err(Error::msg(msg));
            },
        };

        if let Some(name) = updates.name {
            node.base.name = name;
        }
        if let Some(description) = updates.description {
            node.base.description = description;
        }
        if let Some(target) = updates.target {
            node.base.target = target;
        }
        if let Some(node_groups) = updates.node_groups {
            node.base.node_groups = node_groups;
        }
        if let Some(hostname) = updates.hostname {
            node.base.hostname = hostname;
        }  
        if let Some(subnets) = updates.subnets {
            node.base.subnets = subnets;
        }
        if let Some(services) = updates.services {
            node.base.services = services;
        }

        let subnet_relationship_changes = self.update_subnet_node_relationships(&node).await?;
        
        node.updated_at = chrono::Utc::now();

        let node = self.update_default_subnet_membership(node).await?;

        self.storage.update(&node).await?;
        Ok((node, subnet_relationship_changes))
    }

    pub async fn update_default_subnet_membership(&self, mut node: Node) -> Result<Node> {
        let server_ip = self.utils.get_own_ip_address()?;
        let subnet_ids: Vec<Uuid> = node.base.subnets.iter().map(|membership| membership.subnet_id).collect();
        let subnets = self.subnet_service.get_by_ids(subnet_ids).await?;

        node.base.subnets.iter_mut().find_map( |sm| {
            if let Some(cidr) = subnets.iter().find_map(|sub| if sm.subnet_id == sub.id {Some(sub.base.cidr)} else {None}) {
                if cidr.contains(&server_ip) {
                    sm.default = true;
                    return Some(sm.clone())
                }
            }
            None
        });

        Ok(node)
    }

    pub async fn update_subnet_node_relationships(&self, node: &Node) -> Result<NodeSubnetRelationshipChange, Error> {
        let subnet_ids: Vec<Uuid> = node.base.subnets.iter().map(|membership| membership.subnet_id).collect();

        let mut new_gateway: Vec<Subnet> = Vec::new();
        let mut no_longer_gateway: Vec<Subnet>  = Vec::new();
        let mut new_dns_resolver: Vec<Subnet> = Vec::new();
        let mut no_longer_dns_resolver: Vec<Subnet>  = Vec::new();

        if let Ok(mut subnets) = self.subnet_service.get_by_ids(subnet_ids).await {
            subnets.iter_mut().for_each(|subnet| {

                let original_dns_resolver_count = subnet.base.dns_resolvers.len();
                let original_gateway_count = subnet.base.gateways.len();
                
                subnet.base.dns_resolvers = subnet.base.dns_resolvers.iter().filter(|dns_node_id| dns_node_id != &&node.id).cloned().collect();
                subnet.base.gateways = subnet.base.gateways.iter().filter(|gateway_node_id| gateway_node_id != &&node.id).cloned().collect();
                
                subnet.update_node_relationships(node);

                let new_dns_resolver_count = subnet.base.dns_resolvers.len();
                let new_gateway_count = subnet.base.gateways.len();

                if original_dns_resolver_count < new_dns_resolver_count {new_dns_resolver.push(subnet.clone())} else if original_dns_resolver_count > new_dns_resolver_count {no_longer_dns_resolver.push(subnet.clone())}
                if original_gateway_count < new_gateway_count {new_gateway.push(subnet.clone())} else if original_gateway_count > new_gateway_count {no_longer_gateway.push(subnet.clone())}
            });

            let subnet_futures = subnets.into_iter().map(|subnet| self.subnet_service.update_subnet(subnet));
            try_join_all(subnet_futures).await?;
        };

        Ok(NodeSubnetRelationshipChange {
            new_gateway,
            no_longer_gateway,
            new_dns_resolver,
            no_longer_dns_resolver,
        })
    }

    pub async fn delete_node(&self, id: &Uuid) -> Result<()> {

        let mut subnets = self.subnet_service.get_all_subnets().await?;
        let update_futures = subnets
            .iter_mut()
            .filter_map(|s| {
                let has_node_as_dns = s.base.dns_resolvers.iter().find(|n_id| n_id == &id).is_some();
                let has_node_as_gateway = s.base.gateways.iter().find(|n_id| n_id == &id).is_some();
                if has_node_as_dns {
                    s.base.dns_resolvers = s.base.dns_resolvers.iter().filter(|n_id| n_id != &id).cloned().collect();
                }
                if has_node_as_gateway {
                    s.base.gateways = s.base.gateways.iter().filter(|n_id| n_id != &id).cloned().collect();
                }
                if has_node_as_dns || has_node_as_gateway {
                    return Some(self.subnet_service.update_subnet(s.clone()));
                }
                None
            });

        try_join_all(update_futures).await?;

        let all_groups = self.group_storage.get_all().await?;
    
        // Remove node from all groups that contain it
        for mut group in all_groups {
            if group.base.node_sequence.contains(&id) {
                group.base.node_sequence.retain(|seq_id| seq_id != id);
                group.updated_at = chrono::Utc::now();
                self.group_storage.update(&group).await?;
            }
        }

        self.storage.delete(id).await
    }
}