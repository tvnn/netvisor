use async_trait::async_trait;
use sqlx::{SqlitePool};
use crate::types::*;

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

#[async_trait]
pub trait Storage: Send + Sync {
    // Node operations
    async fn get_nodes(&self) -> StorageResult<Vec<NetworkNode>>;
    async fn get_node(&self, id: &str) -> StorageResult<NetworkNode>;
    async fn save_node(&self, node: &NetworkNode) -> StorageResult<()>;
    async fn update_node(&self, id: &str, node: &NetworkNode) -> StorageResult<()>;
    async fn delete_node(&self, id: &str) -> StorageResult<()>;

    // Test operations
    async fn get_tests(&self) -> StorageResult<Vec<Test>>;
    async fn get_test(&self, id: &str) -> StorageResult<Test>;
    async fn save_test(&self, test: &Test) -> StorageResult<()>;
    async fn update_test(&self, id: &str, test: &Test) -> StorageResult<()>;
    async fn delete_test(&self, id: &str) -> StorageResult<()>;

    // Diagnostic results
    async fn save_diagnostic_result(&self, result: &DiagnosticResults) -> StorageResult<()>;
    async fn get_diagnostic_results(&self, test_id: Option<&str>, limit: Option<u32>) -> StorageResult<Vec<DiagnosticResults>>;
    async fn get_latest_diagnostic_result(&self, test_id: &str) -> StorageResult<Option<DiagnosticResults>>;
    async fn delete_diagnostic_result(&self, id: &str) -> StorageResult<()>;
}

pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(database_url: &str) -> StorageResult<Self> {
        // Create database file if it doesn't exist
        if database_url.starts_with("sqlite:") {
            let path = database_url.strip_prefix("sqlite:").unwrap();
            if let Some(parent) = std::path::Path::new(path).parent() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let pool = SqlitePool::connect(database_url).await?;
        
        // Create tables if they don't exist
        // sqlx::query(
        //     r#"
        //     CREATE TABLE IF NOT EXISTS nodes (
        //         id TEXT PRIMARY KEY,
        //         name TEXT NOT NULL,
        //         target TEXT NOT NULL,
        //         description TEXT,
        //         created_at TEXT NOT NULL,
        //         updated_at TEXT NOT NULL
        //     );

        //     CREATE TABLE IF NOT EXISTS tests (
        //         id TEXT PRIMARY KEY,
        //         name TEXT NOT NULL,
        //         description TEXT,
        //         version TEXT NOT NULL,
        //         config TEXT NOT NULL,
        //         created_at TEXT NOT NULL,
        //         updated_at TEXT NOT NULL
        //     );

        //     CREATE TABLE IF NOT EXISTS diagnostic_results (
        //         id TEXT PRIMARY KEY,
        //         test_id TEXT NOT NULL,
        //         test_name TEXT NOT NULL,
        //         results TEXT NOT NULL,
        //         success BOOLEAN NOT NULL,
        //         duration_ms INTEGER NOT NULL,
        //         created_at TEXT NOT NULL,
        //         FOREIGN KEY (test_id) REFERENCES tests(id) ON DELETE CASCADE
        //     );

        //     CREATE INDEX IF NOT EXISTS idx_nodes_created_at ON nodes(created_at);
        //     CREATE INDEX IF NOT EXISTS idx_tests_created_at ON tests(created_at);
        //     CREATE INDEX IF NOT EXISTS idx_diagnostic_results_test_id ON diagnostic_results(test_id);
        //     CREATE INDEX IF NOT EXISTS idx_diagnostic_results_created_at ON diagnostic_results(created_at);
        //     "#,
        // )
        // .execute(&pool)
        // .await?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn get_nodes(&self) -> StorageResult<Vec<NetworkNode>> {
        let rows = sqlx::query!(
            "SELECT id, name, domain, ip, path, port, description, created_at, updated_at FROM nodes ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let nodes = rows
            .into_iter()
            .map(|row| NetworkNode {
                id: row.id,
                name: row.name,
                domain: row.domain,
                ip: row.ip,
                path: row.path,
                port: row.port,
                description: row.description,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
            .collect();

        Ok(nodes)
    }

    async fn get_node(&self, id: &str) -> StorageResult<NetworkNode> {
        let row = sqlx::query!(
            "SELECT id, name, domain, ip, path, port, description, created_at, updated_at FROM nodes WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(NetworkNode {
            id: row.id,
            name: row.name,
            domain: row.domain,
            ip: row.ip,
            path: row.path,
            port: row.port,
            description: row.description,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        })
    }

    async fn save_node(&self, node: &NetworkNode) -> StorageResult<()> {
        let created_at_str = node.created_at.to_rfc3339();
        let updated_at_str = node.updated_at.to_rfc3339();
        sqlx::query!(
            "INSERT INTO nodes (id, name, domain, ip, path, port, description, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            node.id,
            node.name,
            node.domain,
            node.ip,
            node.path,
            node.port,
            node.description,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_node(&self, id: &str, node: &NetworkNode) -> StorageResult<()> {
        let updated_at_str = node.updated_at.to_rfc3339();
        let result = sqlx::query!(
            "UPDATE nodes SET name = ?, domain = ?, ip = ?, path = ?, port = ?, description = ?, updated_at = ? WHERE id = ?",
            node.name,
            node.domain,
            node.ip,
            node.path,
            node.port,
            node.description,
            updated_at_str,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }

    async fn delete_node(&self, id: &str) -> StorageResult<()> {
        let result = sqlx::query!("DELETE FROM nodes WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }

    async fn get_tests(&self) -> StorageResult<Vec<Test>> {
        let rows = sqlx::query!(
            "SELECT id, name, description, layers, created_at, updated_at FROM tests ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let tests = rows
            .into_iter()
            .map(|row| -> StorageResult<Test> {
                let layers: Vec<Layer> = serde_json::from_str(&row.layers)?;
                Ok(Test {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    layers,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                })
            })
            .collect::<StorageResult<Vec<_>>>()?;

        Ok(tests)
    }

    async fn get_test(&self, id: &str) -> StorageResult<Test> {
        let row = sqlx::query!(
            "SELECT id, name, description, layers, created_at, updated_at FROM tests WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        let layers: Vec<Layer> = serde_json::from_str(&row.layers)?;
        
        Ok(Test {
            id: row.id,
            name: row.name,
            description: row.description,
            layers,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        })
    }

    async fn save_test(&self, test: &Test) -> StorageResult<()> {
        let layers = serde_json::to_string(&test.layers)?;
        let created_at_str = test.created_at.to_rfc3339();
        let updated_at_str = test.updated_at.to_rfc3339();
        
        sqlx::query!(
            "INSERT INTO tests (id, name, description, layers, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
            test.id,
            test.name,
            test.description,
            layers,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_test(&self, id: &str, test: &Test) -> StorageResult<()> {
        let layers = serde_json::to_string(&test.layers)?;
        let updated_at_str = test.updated_at.to_rfc3339();
        
        let result = sqlx::query!(
            "UPDATE tests SET name = ?, description = ?, layers = ?, updated_at = ? WHERE id = ?",
            test.name,
            test.description,
            layers,
            updated_at_str,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }

    async fn delete_test(&self, id: &str) -> StorageResult<()> {
        let result = sqlx::query!("DELETE FROM tests WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }

    async fn save_diagnostic_result(&self, result: &DiagnosticResults) -> StorageResult<()> {
        let results_json = serde_json::to_string(&result.layers)?;
        let total_duration_i64 = result.total_duration as i64;
        let timestamp_str = result.timestamp.to_rfc3339();
        
        sqlx::query!(
            "INSERT INTO diagnostic_results (id, test_id, test_name, results, success, duration_ms, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            result.id,
            result.test_id,
            result.test_name,
            results_json,
            result.success,
            total_duration_i64,
            timestamp_str
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_diagnostic_results(&self, test_id: Option<&str>, limit: Option<u32>) -> StorageResult<Vec<DiagnosticResults>> {
        let limit = limit.unwrap_or(100) as i64;
        
        // Use the same query structure for both cases
        let test_id_param = test_id.unwrap_or("");
        let use_test_filter = test_id.is_some();
        let bit = if use_test_filter { 1 } else { 0 };
        
        let rows = sqlx::query!(
            "SELECT id, test_id, test_name, results, success, duration_ms, created_at 
            FROM diagnostic_results 
            WHERE (? = 0 OR test_id = ?) 
            ORDER BY created_at DESC LIMIT ?",
            bit,
            test_id_param,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|row| -> StorageResult<DiagnosticResults> {
                let layers: Vec<LayerResult> = serde_json::from_str(&row.results)?;
                Ok(DiagnosticResults {
                    id: row.id,
                    test_id: row.test_id,
                    test_name: row.test_name,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    layers,
                    success: row.success,
                    total_duration: row.duration_ms as u64,
                })
            })
            .collect::<StorageResult<Vec<_>>>()?;

        Ok(results)
    }

    async fn get_latest_diagnostic_result(&self, test_id: &str) -> StorageResult<Option<DiagnosticResults>> {
        let row = sqlx::query!(
            "SELECT id, test_id, test_name, results, success, duration_ms, created_at 
             FROM diagnostic_results WHERE test_id = ? ORDER BY created_at DESC LIMIT 1",
            test_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let layers: Vec<LayerResult> = serde_json::from_str(&row.results)?;
            Ok(Some(DiagnosticResults {
                id: row.id,
                test_id: row.test_id,
                test_name: row.test_name,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                layers,
                success: row.success,
                total_duration: row.duration_ms as u64,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete_diagnostic_result(&self, id: &str) -> StorageResult<()> {
        let result = sqlx::query!("DELETE FROM diagnostic_results WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }
}