use anyhow::Result;
use sqlx::SqlitePool;

use super::seed_data::{create_internet_connectivity_host, create_public_dns_host};
use crate::server::hosts::storage::HostStorage;
use crate::server::subnets::storage::SubnetStorage;
use crate::server::{
    hosts::storage::SqliteHostStorage,
    services::storage::{ServiceStorage, SqliteServiceStorage},
    shared::storage::seed_data::{create_remote_host, create_remote_subnet, create_wan_subnet},
    subnets::storage::SqliteSubnetStorage,
};
pub struct DatabaseMigrations;

impl DatabaseMigrations {
    /// Initialize database with current schema
    pub async fn initialize(pool: &SqlitePool) -> Result<()> {
        tracing::info!("Initializing database schema...");

        sqlx::migrate!("./migrations").run(pool).await?;

        tracing::info!("Database schema initialized successfully");

        Self::seed_default_data(pool).await?;

        Ok(())
    }

    async fn seed_default_data(pool: &SqlitePool) -> Result<()> {
        // Check if hosts already exist
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hosts")
            .fetch_one(pool)
            .await?;

        if count.0 > 0 {
            tracing::info!("Database already contains data, skipping seed data");
            return Ok(());
        }

        tracing::info!("Seeding default data...");

        // Use actual compiled structs
        let mut wan_subnet = create_wan_subnet();
        let mut remote_subnet = create_remote_subnet();
        let (dns_host, dns_service) = create_public_dns_host(&wan_subnet);
        let (web_host, web_service) = create_internet_connectivity_host(&wan_subnet);
        let (remote_host, client_service) = create_remote_host(&remote_subnet);

        wan_subnet.create_host_relationship(&dns_host);
        wan_subnet.create_host_relationship(&web_host);
        remote_subnet.create_host_relationship(&remote_host);

        let host_storage = SqliteHostStorage::new(pool.clone());
        let service_storage = SqliteServiceStorage::new(pool.clone());
        let subnet_storage = SqliteSubnetStorage::new(pool.clone());

        subnet_storage.create(&wan_subnet).await?;
        subnet_storage.create(&remote_subnet).await?;
        host_storage.create(&dns_host).await?;
        host_storage.create(&web_host).await?;
        host_storage.create(&remote_host).await?;
        service_storage.create(&dns_service).await?;
        service_storage.create(&web_service).await?;
        service_storage.create(&client_service).await?;

        tracing::info!("Default hosts seeded successfully");
        Ok(())
    }
}
