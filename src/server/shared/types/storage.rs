use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;


use crate::server::{
    daemons::storage::{DaemonStorage, SqliteDaemonStorage}, node_groups::storage::{NodeGroupStorage, SqliteNodeGroupStorage}, nodes::storage::{NodeStorage, SqliteNodeStorage}, shared::storage::DatabaseMigrations, subnets::storage::{SqliteSubnetStorage, SubnetStorage}
};

pub struct StorageFactory {
    pub nodes: Arc<dyn NodeStorage>,
    pub node_groups: Arc<dyn NodeGroupStorage>,
    pub daemons: Arc<dyn DaemonStorage>,
    pub subnets: Arc<dyn SubnetStorage>
}

impl StorageFactory {
    pub async fn new_sqlite(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Initialize database schema
        DatabaseMigrations::initialize(&pool).await?;
        
        Ok(Self {
            nodes: Arc::new(SqliteNodeStorage::new(pool.clone())),
            node_groups: Arc::new(SqliteNodeGroupStorage::new(pool.clone())),
            daemons: Arc::new(SqliteDaemonStorage::new(pool.clone())),
            subnets: Arc::new(SqliteSubnetStorage::new(pool.clone()))
        })
    }
}