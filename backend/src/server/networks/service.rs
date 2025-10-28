use crate::{
    daemon::runtime::types::InitializeDaemonRequest,
    server::{
        hosts::service::HostService,
        networks::{storage::NetworkStorage, types::Network},
        shared::storage::seed_data::{
            create_internet_connectivity_host, create_public_dns_host, create_remote_host,
            create_remote_subnet, create_wan_subnet,
        },
        subnets::service::SubnetService,
    },
};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct NetworkService {
    network_storage: Arc<dyn NetworkStorage>,
    host_service: Arc<HostService>,
    subnet_service: Arc<SubnetService>,
    integrated_daemon_url: Option<String>,
}

impl NetworkService {
    pub fn new(
        network_storage: Arc<dyn NetworkStorage>,
        host_service: Arc<HostService>,
        subnet_service: Arc<SubnetService>,
        integrated_daemon_url: Option<String>,
    ) -> Self {
        Self {
            network_storage,
            host_service,
            subnet_service,
            integrated_daemon_url,
        }
    }

    /// Create a new network
    pub async fn create_network(&self, network: Network) -> Result<Network> {
        self.network_storage.create(&network).await?;

        self.seed_default_data(network.id).await?;

        self.notify_local_daemon(network.id).await?;

        tracing::info!("Created network {}: {}", network.base.name, network.id);
        Ok(network)
    }

    async fn notify_local_daemon(&self, network_id: Uuid) -> Result<()> {
        let daemon_url = self
            .integrated_daemon_url
            .clone()
            .unwrap_or("http://daemon:60073".to_string());

        let client = reqwest::Client::new();

        match client
            .post(format!("{}/api/initialize", daemon_url))
            .json(&InitializeDaemonRequest { network_id })
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status();

                if status.is_success() {
                    tracing::info!("Successfully initialized daemon");
                } else {
                    let body = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read body".to_string());
                    tracing::warn!("Daemon returned error. Status: {}, Body: {}", status, body);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to reach daemon: {:?}", e);
            }
        }

        Ok(())
    }

    /// Get network by ID
    pub async fn get_network(&self, id: &Uuid) -> Result<Option<Network>> {
        self.network_storage.get_by_id(id).await
    }

    pub async fn get_default_network(&self, user_id: &Uuid) -> Result<Option<Network>> {
        let all_networks = self.get_all_networks(user_id).await?;
        Ok(all_networks
            .into_iter()
            .find(|n| n.base.is_default && n.base.user_id == *user_id))
    }

    /// Get all networks
    pub async fn get_all_networks(&self, user_id: &Uuid) -> Result<Vec<Network>> {
        self.network_storage.get_all(user_id).await
    }

    /// Update network
    pub async fn update_network(&self, mut network: Network) -> Result<Network> {
        let now = chrono::Utc::now();
        network.updated_at = now;

        self.network_storage.update(&network).await?;

        tracing::info!("Updated network {}: {}", network.base.name, network.id);
        Ok(network)
    }

    /// Delete network
    pub async fn delete_network(&self, id: &Uuid) -> Result<()> {
        // Get group to find hosts to update
        let network = self
            .get_network(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Network not found"))?;

        self.network_storage.delete(id).await?;
        tracing::info!("Deleted network {}: {}", network.base.name, network.id);
        Ok(())
    }

    pub async fn seed_default_data(&self, network_id: Uuid) -> Result<()> {
        tracing::info!("Seeding default data...");

        let wan_subnet = create_wan_subnet(network_id);
        let remote_subnet = create_remote_subnet(network_id);
        let (dns_host, dns_service) = create_public_dns_host(&wan_subnet, network_id);
        let (web_host, web_service) = create_internet_connectivity_host(&wan_subnet, network_id);
        let (remote_host, client_service) = create_remote_host(&remote_subnet, network_id);

        self.subnet_service.create_subnet(wan_subnet).await?;
        self.subnet_service.create_subnet(remote_subnet).await?;
        self.host_service
            .create_host_with_services(dns_host, vec![dns_service])
            .await?;
        self.host_service
            .create_host_with_services(web_host, vec![web_service])
            .await?;
        self.host_service
            .create_host_with_services(remote_host, vec![client_service])
            .await?;

        tracing::info!("Default data seeded successfully");

        Ok(())
    }
}
