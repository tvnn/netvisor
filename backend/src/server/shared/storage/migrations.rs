use anyhow::Result;
use sqlx::PgPool;

pub struct DatabaseMigrations;

impl DatabaseMigrations {
    /// Initialize database with current schema
    pub async fn initialize(pool: &PgPool) -> Result<()> {
        tracing::info!("Initializing database schema...");

        sqlx::migrate!("./migrations").run(pool).await?;

        tracing::info!("Database schema initialized successfully");

        Ok(())
    }
}
