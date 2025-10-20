use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;

use crate::server::{
    daemons::storage::{DaemonStorage, PostgresDaemonStorage},
    groups::storage::{GroupStorage, PostgresGroupStorage},
    hosts::storage::{HostStorage, PostgresHostStorage},
    services::storage::{ServiceStorage, PostgresServiceStorage},
    shared::storage::DatabaseMigrations,
    subnets::storage::{PostgresSubnetStorage, SubnetStorage},
};

pub struct StorageFactory {
    pub hosts: Arc<dyn HostStorage>,
    pub host_groups: Arc<dyn GroupStorage>,
    pub daemons: Arc<dyn DaemonStorage>,
    pub subnets: Arc<dyn SubnetStorage>,
    pub services: Arc<dyn ServiceStorage>,
}

impl StorageFactory {
    pub async fn new(database_url: &str) -> Result<Self> {
        
        let pool = PgPool::connect(database_url).await?;

        // Initialize database schema
        DatabaseMigrations::initialize(&pool).await?;

        Ok(Self {
            hosts: Arc::new(PostgresHostStorage::new(pool.clone())),
            host_groups: Arc::new(PostgresGroupStorage::new(pool.clone())),
            daemons: Arc::new(PostgresDaemonStorage::new(pool.clone())),
            subnets: Arc::new(PostgresSubnetStorage::new(pool.clone())),
            services: Arc::new(PostgresServiceStorage::new(pool.clone())),
        })
    }
}
