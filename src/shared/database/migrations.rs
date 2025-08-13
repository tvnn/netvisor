use sqlx::SqlitePool;
use anyhow::Result;

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
        Ok(())
    }
}