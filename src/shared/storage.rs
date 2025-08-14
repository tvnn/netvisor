use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;


use crate::components::{
    nodes::storage::{NodeStorage, SqliteNodeStorage},
    node_groups::storage::{NodeGroupStorage, SqliteNodeGroupStorage},
    // diagnostics::storage::{DiagnosticStorage, SqliteDiagnosticStorage},
};
use super::database::DatabaseMigrations;

pub struct StorageFactory {
    pub nodes: Arc<dyn NodeStorage>,
    pub node_groups: Arc<dyn NodeGroupStorage>,
    // pub diagnostics: Arc<dyn DiagnosticStorage>,
}

impl StorageFactory {
    pub async fn new_sqlite(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Initialize database schema
        DatabaseMigrations::initialize(&pool).await?;
        
        Ok(Self {
            nodes: Arc::new(SqliteNodeStorage::new(pool.clone())),
            node_groups: Arc::new(SqliteNodeGroupStorage::new(pool.clone())),
            // diagnostics: Arc::new(SqliteDiagnosticStorage::new(pool)),
        })
    }
}

// // Placeholder for diagnostic storage - will implement later
// pub struct SqliteDiagnosticStorage {
//     pool: SqlitePool,
// }

// impl SqliteDiagnosticStorage {
//     pub fn new(pool: SqlitePool) -> Self {
//         Self { pool }
//     }
// }

// #[async_trait]
// pub trait DiagnosticStorage: Send + Sync {
//     async fn create(&self, execution: &DiagnosticExecution) -> Result<()>;
//     async fn get_by_id(&self, id: &str) -> Result<Option<DiagnosticExecution>>;
//     async fn get_all(&self) -> Result<Vec<DiagnosticExecution>>;
//     async fn get_by_group(&self, group_id: &str) -> Result<Vec<DiagnosticExecution>>;
// }

// #[async_trait]
// impl DiagnosticStorage for SqliteDiagnosticStorage {
//     async fn create(&self, _execution: &DiagnosticExecution) -> Result<()> {
//         // TODO: Implement diagnostic storage
//         Ok(())
//     }

//     async fn get_by_id(&self, _id: &str) -> Result<Option<DiagnosticExecution>> {
//         // TODO: Implement diagnostic storage
//         Ok(None)
//     }

//     async fn get_all(&self) -> Result<Vec<DiagnosticExecution>> {
//         // TODO: Implement diagnostic storage
//         Ok(Vec::new())
//     }

//     async fn get_by_group(&self, _group_id: &str) -> Result<Vec<DiagnosticExecution>> {
//         // TODO: Implement diagnostic storage
//         Ok(Vec::new())
//     }
// }