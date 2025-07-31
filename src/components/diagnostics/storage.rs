use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::shared::storage::{StorageResult,StorageError};
use super::types::{DiagnosticResults, LayerResult};

#[async_trait]
pub trait DiagnosticStorage: Send + Sync {
    async fn save_diagnostic_result(&self, result: &DiagnosticResults) -> StorageResult<()>;
    async fn get_diagnostic_results(&self, test_id: Option<&str>, limit: Option<u32>) -> StorageResult<Vec<DiagnosticResults>>;
    async fn get_latest_diagnostic_result(&self, test_id: &str) -> StorageResult<Option<DiagnosticResults>>;
    async fn delete_diagnostic_result(&self, id: &str) -> StorageResult<()>;
}

pub struct SqliteDiagnosticStorage {
    pool: SqlitePool,
}

impl SqliteDiagnosticStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DiagnosticStorage for SqliteDiagnosticStorage {
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
}