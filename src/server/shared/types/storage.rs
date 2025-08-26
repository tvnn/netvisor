use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;


use crate::server::{
    daemons::storage::{DaemonStorage, SqliteDaemonStorage}, diagnostics::storage::{DiagnosticStorage, SqliteDiagnosticStorage}, node_groups::storage::{NodeGroupStorage, SqliteNodeGroupStorage}, nodes::storage::{NodeStorage, SqliteNodeStorage}, shared::storage::DatabaseMigrations
};

pub struct StorageFactory {
    pub nodes: Arc<dyn NodeStorage>,
    pub node_groups: Arc<dyn NodeGroupStorage>,
    pub diagnostics: Arc<dyn DiagnosticStorage>,
    pub daemons: Arc<dyn DaemonStorage>
}

impl StorageFactory {
    pub async fn new_sqlite(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Initialize database schema
        DatabaseMigrations::initialize(&pool).await?;
        
        Ok(Self {
            nodes: Arc::new(SqliteNodeStorage::new(pool.clone())),
            node_groups: Arc::new(SqliteNodeGroupStorage::new(pool.clone())),
            diagnostics: Arc::new(SqliteDiagnosticStorage::new(pool.clone())),
            daemons: Arc::new(SqliteDaemonStorage::new(pool.clone()))
        })
    }
}