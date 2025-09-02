use anyhow::Result;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{capabilities::types::base::CapabilityDiscriminants, nodes::{handlers::NodeSubnetRelationshipChange, types::base::Node}, subnets::{storage::SubnetStorage, types::base::Subnet}
};

pub struct SubnetService {
    storage: Arc<dyn SubnetStorage>,
}

impl SubnetService {
    pub fn new(storage: Arc<dyn SubnetStorage>) -> Self {
        Self { 
            storage,
        }
    }

    /// Create a new subnet
    pub async fn create_subnet(&self, subnet: Subnet) -> Result<Subnet> {
        
        let all_subnets = self.storage.get_all().await?;

        if let Some(same_subnet) = all_subnets.iter().find(|s| subnet.eq(s)) {
            Ok(same_subnet.clone())
        } else {
            self.storage.create(&subnet).await?;
            Ok(subnet)
        }
    }

    pub async fn get_subnet(&self, id: &Uuid) -> Result<Option<Subnet>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_subnets(&self) -> Result<Vec<Subnet>> {
        self.storage.get_all().await
    }

    pub async fn update_subnet(&self, mut subnet: Subnet) -> Result<Subnet> {
        subnet.updated_at = chrono::Utc::now();
        self.storage.update(&subnet).await?;
        Ok(subnet)
    }

    pub async fn update_subnet_node_relationships(&self, node: &Node) -> NodeSubnetRelationshipChange {
        let subnet_ids: Vec<Uuid> = node.base.subnets.iter().map(|membership| membership.subnet_id).collect();

        let node_has_dns_capability = node.has_capability(CapabilityDiscriminants::Dns);

        let mut new_gateway: Vec<Subnet> = Vec::new();
        let mut no_longer_gateway: Vec<Subnet>  = Vec::new();
        let mut new_dns_resolver: Vec<Subnet> = Vec::new();
        let mut no_longer_dns_resolver: Vec<Subnet>  = Vec::new();

        if let Ok(mut subnets) = self.storage.get_by_ids(subnet_ids).await {
            subnets.iter_mut().for_each(|subnet| {

                let original_dns_resolver_count = subnet.base.dns_resolvers.len();
                let original_gateway_count = subnet.base.gateways.len();
                
                subnet.base.dns_resolvers = subnet.base.dns_resolvers.iter().filter(|dns_node_id| dns_node_id != &&node.id).cloned().collect();
                subnet.base.gateways = subnet.base.gateways.iter().filter(|gateway_node_id| gateway_node_id != &&node.id).cloned().collect();
                
                if node_has_dns_capability {subnet.base.dns_resolvers.push(node.id)}
                if node.is_gateway_for_subnet(subnet) {subnet.base.gateways.push(node.id)}

                let new_dns_resolver_count = subnet.base.dns_resolvers.len();
                let new_gateway_count = subnet.base.gateways.len();

                if original_dns_resolver_count < new_dns_resolver_count {new_dns_resolver.push(subnet.clone())} else if original_dns_resolver_count > new_dns_resolver_count {no_longer_dns_resolver.push(subnet.clone())}
                if original_gateway_count < new_gateway_count {new_gateway.push(subnet.clone())} else if original_gateway_count > new_gateway_count {no_longer_gateway.push(subnet.clone())}
            });
        };

        NodeSubnetRelationshipChange {
            new_gateway,
            no_longer_gateway,
            new_dns_resolver,
            no_longer_dns_resolver,
        }
    }

    pub async fn delete_subnet(&self, id: &Uuid) -> Result<()> {
        self.storage.delete(id).await
    }
}