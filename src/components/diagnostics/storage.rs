use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use anyhow::Result;
use crate::components::nodes::types::tests::NodeTestResults;

use super::types::*;

#[async_trait]
pub trait DiagnosticStorage: Send + Sync {
    async fn create(&self, execution: DiagnosticExecution) -> Result<DiagnosticExecution>;
    async fn get(&self, id: &str) -> Result<Option<DiagnosticExecution>>;
    async fn update(&self, execution: DiagnosticExecution) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn get_all(&self) -> Result<Vec<DiagnosticExecution>>;
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
    async fn create(&self, execution: DiagnosticExecution) -> Result<DiagnosticExecution> {
        let node_results_json = serde_json::to_string(&execution.node_results)?;
        let status_str = serde_json::to_string(&execution.status)?;
        let started_at_str = execution.started_at.to_rfc3339();
        let completed_at_str = execution.completed_at.map(|dt| dt.to_rfc3339());

        sqlx::query(
            r#"
            INSERT INTO diagnostic_executions (
                id, group_id, trigger_reason, node_results, 
                status, generated_remediation_id, started_at, 
                completed_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&execution.id)
        .bind(&execution.base.group_id)
        .bind(serde_json::to_string(&execution.base.trigger_reason)?)
        .bind(&node_results_json)
        .bind(&status_str)
        .bind(&execution.generated_remediation_id)
        .bind(&started_at_str)
        .bind(&completed_at_str)
        .execute(&self.pool)
        .await?;

        Ok(execution)
    }

    async fn get(&self, id: &str) -> Result<Option<DiagnosticExecution>> {
        let row = sqlx::query("SELECT * FROM diagnostic_executions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_diagnostic_execution(row)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, execution: DiagnosticExecution) -> Result<()> {
        let node_results_json = serde_json::to_string(&execution.node_results)?;
        let status_str = serde_json::to_string(&execution.status)?;
        let completed_at_str = execution.completed_at.map(|dt| dt.to_rfc3339());
        let updated_at_str = chrono::Utc::now().to_rfc3339(); // Add this line

        sqlx::query(
            r#"
            UPDATE diagnostic_executions SET
                trigger_reason = ?, node_results = ?,
                status = ?, generated_remediation_id = ?,
                completed_at = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(serde_json::to_string(&execution.base.trigger_reason)?)
        .bind(&node_results_json)
        .bind(&status_str)
        .bind(&execution.generated_remediation_id)
        .bind(&completed_at_str)
        .bind(&updated_at_str) // Add this binding
        .bind(&execution.id) // Move this to the end to match WHERE clause
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM diagnostic_executions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<DiagnosticExecution>> {
        let rows = sqlx::query("SELECT * FROM diagnostic_executions ORDER BY started_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(row_to_diagnostic_execution(row)?);
        }

        Ok(executions)
    }
}

fn row_to_diagnostic_execution(row: sqlx::sqlite::SqliteRow) -> Result<DiagnosticExecution> {
    let node_results_json: String = row.get("node_results");
    let status_json: String = row.get("status");
    let trigger_reason_json: String = row.get("trigger_reason");

    let started_at_str: String = row.get("started_at");
    let completed_at_str: Option<String> = row.get("completed_at");

    let trigger_reason: DiagnosticTrigger = serde_json::from_str(&trigger_reason_json)?;
    let node_results: Vec<NodeTestResults> = serde_json::from_str(&node_results_json)?;
    let status: DiagnosticStatus = serde_json::from_str(&status_json)?;
    let started_at = chrono::DateTime::parse_from_rfc3339(&started_at_str)?.with_timezone(&chrono::Utc);
    let completed_at = completed_at_str
        .map(|s| chrono::DateTime::parse_from_rfc3339(&s))
        .transpose()?
        .map(|dt| dt.with_timezone(&chrono::Utc));

    Ok(DiagnosticExecution {
        id: row.get("id"),
        base: DiagnosticExecutionBase { 
            group_id: row.get("group_id"), 
            trigger_reason
        },
        node_results,
        status,
        generated_remediation_id: row.get("generated_remediation_id"),
        started_at,
        completed_at,
    })
}