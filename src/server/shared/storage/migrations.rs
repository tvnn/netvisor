use sqlx::SqlitePool;
use anyhow::Result;

use crate::{server::hosts::storage::SqliteHostStorage};
use super::seed_data::{create_internet_connectivity_host, create_public_dns_host};
use crate::server::hosts::storage::HostStorage;
pub struct DatabaseMigrations;

impl DatabaseMigrations {
    /// Initialize database with current schema
    pub async fn initialize(pool: &SqlitePool) -> Result<()> {
        tracing::info!("Initializing database schema...");
        
        // Create all tables from schema
        let schema = include_str!("schema.sql");
        
        // Split on semicolons and execute each statement
        for statement in schema.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("--") {
                sqlx::query(statement).execute(pool).await?;
            }
        }
        
        tracing::info!("Database schema initialized successfully");
        
        Self::seed_default_hosts(pool).await?;

        Ok(())
    }

    async fn seed_default_hosts(pool: &SqlitePool) -> Result<()> {
        // Check if hosts already exist
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hosts")
            .fetch_one(pool)
            .await?;
            
        if count.0 > 0 {
            tracing::info!("Database already contains hosts, skipping seed data");
            return Ok(());
        }
        
        tracing::info!("Seeding default hosts...");
        
        // Use actual compiled structs
        let dns_host = create_public_dns_host();
        let connectivity_host = create_internet_connectivity_host();
        let host_storage = SqliteHostStorage::new(pool.clone());
        
        host_storage.create(&dns_host).await?;
        host_storage.create(&connectivity_host).await?;

        
        tracing::info!("Default hosts seeded successfully");
        Ok(())
    }
}