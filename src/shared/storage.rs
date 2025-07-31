// src/shared/storage.rs
use crate::components::{
    nodes::storage::{NodeStorage, SqliteNodeStorage}, 
    tests::storage::{TestStorage, SqliteTestStorage},
    diagnostics::storage::{DiagnosticStorage, SqliteDiagnosticStorage},
};

pub struct StorageFactory {
    pub nodes: Box<dyn NodeStorage>,
    pub tests: Box<dyn TestStorage>, 
    pub diagnostics: Box<dyn DiagnosticStorage>,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type StorageResult<T> = Result<T, StorageError>;

impl StorageFactory {
    pub async fn new_sqlite(database_url: &str) -> StorageResult<Self> {
        // Create database file if it doesn't exist
        let path = database_url.strip_prefix("sqlite:").unwrap();
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let pool = sqlx::SqlitePool::connect(database_url).await?;
        
        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                domain TEXT,
                ip TEXT,
                path TEXT,
                port INTEGER,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tests (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                layers TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS diagnostic_results (
                id TEXT PRIMARY KEY,
                test_id TEXT NOT NULL,
                test_name TEXT NOT NULL,
                results TEXT NOT NULL,
                success BOOLEAN NOT NULL,
                duration_ms INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (test_id) REFERENCES tests(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_nodes_created_at ON nodes(created_at);
            CREATE INDEX IF NOT EXISTS idx_tests_created_at ON tests(created_at);
            CREATE INDEX IF NOT EXISTS idx_diagnostic_results_test_id ON diagnostic_results(test_id);
            CREATE INDEX IF NOT EXISTS idx_diagnostic_results_created_at ON diagnostic_results(created_at);
            "#,
        )
        .execute(&pool)
        .await?;
        
        Ok(Self {
            nodes: Box::new(SqliteNodeStorage::new(pool.clone())),
            tests: Box::new(SqliteTestStorage::new(pool.clone())),
            diagnostics: Box::new(SqliteDiagnosticStorage::new(pool)),
        })
    }
}