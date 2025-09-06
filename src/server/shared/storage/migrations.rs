use sqlx::SqlitePool;
use anyhow::Result;

use crate::{server::nodes::storage::SqliteNodeStorage};
use super::seed_data::{create_internet_connectivity_node, create_public_dns_node};
use crate::server::nodes::storage::NodeStorage;
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
        
        Self::seed_default_nodes(pool).await?;

        Ok(())
    }

    async fn seed_default_nodes(pool: &SqlitePool) -> Result<()> {
        // Check if nodes already exist
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM nodes")
            .fetch_one(pool)
            .await?;
            
        if count.0 > 0 {
            tracing::info!("Database already contains nodes, skipping seed data");
            return Ok(());
        }
        
        tracing::info!("Seeding default nodes...");
        
        // Use actual compiled structs
        let dns_node = create_public_dns_node();
        let connectivity_node = create_internet_connectivity_node();
        let node_storage = SqliteNodeStorage::new(pool.clone());
        
        node_storage.create(&dns_node).await?;
        node_storage.create(&connectivity_node).await?;

        
        tracing::info!("Default nodes seeded successfully");
        Ok(())
    }
}