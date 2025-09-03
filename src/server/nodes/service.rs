use anyhow::{Error, Result};
use futures::future::join_all;
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};
use strum::IntoDiscriminant;
use crate::server::{
    capabilities::types::base::CapabilityDiscriminants, node_groups::storage::NodeGroupStorage, nodes::{
        storage::NodeStorage,
        types::{
            api::{NodeCapabilityTestChange, NodeSubnetRelationshipChange, NodeUpdateRequest}, base::{Node, NodeBase}, tests::NodeTestResults
        }
    }, subnets::{storage::SubnetStorage, types::base::Subnet}, tests::{service::TestService, types::execution::{TestResult, Timer}}
};

pub struct NodeService {
    storage: Arc<dyn NodeStorage>,
    group_storage: Arc<dyn NodeGroupStorage>,
    subnet_storage: Arc<dyn SubnetStorage>,
    test_service: TestService,
}

impl NodeService {
    pub fn new(storage: Arc<dyn NodeStorage>, group_storage: Arc<dyn NodeGroupStorage>, subnet_storage: Arc<dyn SubnetStorage>) -> Self {
        Self { 
            storage,
            group_storage,
            subnet_storage,
            test_service: TestService::new(),
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
                self.storage.create(&node).await?;
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

    pub async fn update_node(&self, id: &Uuid, updates: NodeUpdateRequest) -> Result<(Node, HashMap<CapabilityDiscriminants, NodeCapabilityTestChange>, NodeSubnetRelationshipChange), Error> {
        
        let mut node = match self.get_node(&id).await? {
            Some(n) => n,
            None => {
                let msg = format!("Node '{}' not found", id);
                return Err(Error::msg(msg));
            },
        };

        let mut capability_test_changes: HashMap<CapabilityDiscriminants, NodeCapabilityTestChange> = HashMap::new();

        if let Some(name) = updates.name {
            node.base.name = name;
        }
        if let Some(node_type) = updates.node_type {
            node.base.node_type = node_type;
        }
        if let Some(description) = updates.description {
            node.base.description = description;
        }
        if let Some(target) = updates.target {
            node.base.target = target;
        }
        if let Some(dns_resolver_node_id) = updates.dns_resolver_node_id {
            node.base.dns_resolver_node_id = dns_resolver_node_id;
        }
        if let Some(discovery_status) = updates.discovery_status {
            node.base.discovery_status = discovery_status;
        }
        if let Some(status) = updates.status {
            node.base.status = status;
        }
        if let Some(monitoring_interval) = updates.monitoring_interval {
            node.base.monitoring_interval = monitoring_interval;
        }
        if let Some(node_groups) = updates.node_groups {
            node.base.node_groups = node_groups;
        }
        if let Some(hostname) = updates.hostname {
            node.base.hostname = hostname;
        }  

        let validated_capabilities = node.base.capabilities.iter().map(|cap| {
            let (newly_compatible, incompatible) = cap.validate_node_capability_test_compatibility(node.as_context());
            
            let mut capability = cap.clone();
            capability.config_base_mut().remove_tests(incompatible.clone());
            capability.config_base_mut().add_tests(newly_compatible.clone());

            capability_test_changes.insert(capability.discriminant(), NodeCapabilityTestChange {
                newly_compatible: newly_compatible.iter().map(|ct| ct.test.discriminant()).collect(),
                incompatible
            });

            capability
        }).collect();

        node.base.capabilities = validated_capabilities;

        let subnet_relationship_changes = self.update_subnet_node_relationships(&node).await;
        
        node.updated_at = chrono::Utc::now();
        self.storage.update(&node).await?;
        Ok((node, capability_test_changes, subnet_relationship_changes))
    }

    pub async fn update_subnet_node_relationships(&self, node: &Node) -> NodeSubnetRelationshipChange {
        let subnet_ids: Vec<Uuid> = node.base.subnets.iter().map(|membership| membership.subnet_id).collect();

        let mut new_gateway: Vec<Subnet> = Vec::new();
        let mut no_longer_gateway: Vec<Subnet>  = Vec::new();
        let mut new_dns_resolver: Vec<Subnet> = Vec::new();
        let mut no_longer_dns_resolver: Vec<Subnet>  = Vec::new();

        if let Ok(mut subnets) = self.subnet_storage.get_by_ids(subnet_ids).await {
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

            let subnet_futures = subnets.iter().map(|subnet| self.subnet_storage.update(&subnet));
            futures::future::join_all(subnet_futures).await;
        };

        NodeSubnetRelationshipChange {
            new_gateway,
            no_longer_gateway,
            new_dns_resolver,
            no_longer_dns_resolver,
        }
    }

    pub async fn delete_node(&self, id: &Uuid) -> Result<()> {

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

    pub async fn execute_tests(&self, node: &mut Node) -> NodeTestResults {
        
        let timer = Timer::now();

        let test_futures: Vec<_> = node.base.capabilities.iter()
            .flat_map(|capability| 
                capability.config_base().tests.iter()
                    .filter(|test| test.enabled)
                    .map(|test| 
                        self.test_service.execute_test(test, node, capability, &self)
                    )
            )
            .collect();

        let test_results: Vec<TestResult> = join_all(test_futures).await;

        node.update_status_from_tests(&test_results);

        NodeTestResults {
            test_results,
            node_id: node.id,
            node_status: node.base.status.clone(),
            duration_ms: timer.elapsed_ms(),
            executed_at: timer.datetime(),
        }
    }
}