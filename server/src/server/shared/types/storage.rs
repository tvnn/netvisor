use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;


use crate::server::{
    daemons::storage::{DaemonStorage, SqliteDaemonStorage}, groups::storage::{GroupStorage, SqliteGroupStorage}, hosts::storage::{HostStorage, SqliteHostStorage}, services::storage::{ServiceStorage, SqliteServiceStorage}, shared::storage::DatabaseMigrations, subnets::storage::{SqliteSubnetStorage, SubnetStorage}
};

pub struct StorageFactory {
    pub hosts: Arc<dyn HostStorage>,
    pub host_groups: Arc<dyn GroupStorage>,
    pub daemons: Arc<dyn DaemonStorage>,
    pub subnets: Arc<dyn SubnetStorage>,
    pub services: Arc<dyn ServiceStorage>
}

impl StorageFactory {
    pub async fn new_sqlite(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Initialize database schema
        DatabaseMigrations::initialize(&pool).await?;
        
        Ok(Self {
            hosts: Arc::new(SqliteHostStorage::new(pool.clone())),
            host_groups: Arc::new(SqliteGroupStorage::new(pool.clone())),
            daemons: Arc::new(SqliteDaemonStorage::new(pool.clone())),
            subnets: Arc::new(SqliteSubnetStorage::new(pool.clone())),
            services: Arc::new(SqliteServiceStorage::new(pool.clone()))
        })
    }
}