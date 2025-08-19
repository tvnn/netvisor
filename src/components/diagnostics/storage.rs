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
    async fn get_by_group(&self, group_id: &str) -> Result<Vec<DiagnosticExecution>>;
    async fn get_by_status(&self, status: DiagnosticStatus) -> Result<Vec<DiagnosticExecution>>;
    async fn get_recent(&self, limit: usize) -> Result<Vec<DiagnosticExecution>>;
    async fn get_with_filters(&self, query: DiagnosticListQuery) -> Result<Vec<DiagnosticExecution>>;
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
        let created_at_str = execution.created_at.to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO diagnostic_executions (
                id, group_id, trigger_reason, node_results, 
                status, generated_remediation_id, started_at, 
                completed_at, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(&created_at_str)
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
        .bind(&execution.id)
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
        let rows = sqlx::query("SELECT * FROM diagnostic_executions ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(row_to_diagnostic_execution(row)?);
        }

        Ok(executions)
    }

    async fn get_by_group(&self, group_id: &str) -> Result<Vec<DiagnosticExecution>> {
        let rows = sqlx::query("SELECT * FROM diagnostic_executions WHERE group_id = ? ORDER BY created_at DESC")
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(row_to_diagnostic_execution(row)?);
        }

        Ok(executions)
    }

    async fn get_by_status(&self, status: DiagnosticStatus) -> Result<Vec<DiagnosticExecution>> {
        let status_str = serde_json::to_string(&status)?;
        let rows = sqlx::query("SELECT * FROM diagnostic_executions WHERE status = ? ORDER BY created_at DESC")
            .bind(&status_str)
            .fetch_all(&self.pool)
            .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(row_to_diagnostic_execution(row)?);
        }

        Ok(executions)
    }

    async fn get_recent(&self, limit: usize) -> Result<Vec<DiagnosticExecution>> {
        let rows = sqlx::query("SELECT * FROM diagnostic_executions ORDER BY created_at DESC LIMIT ?")
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(row_to_diagnostic_execution(row)?);
        }

        Ok(executions)
    }

    async fn get_with_filters(&self, query: DiagnosticListQuery) -> Result<Vec<DiagnosticExecution>> {
        let mut sql = "SELECT * FROM diagnostic_executions WHERE 1=1".to_string();
        let mut params: Vec<String> = Vec::new();

        if let Some(group_id) = &query.group_id {
            sql.push_str(" AND group_id = ?");
            params.push(group_id.clone());
        }

        if let Some(status) = &query.status {
            sql.push_str(" AND status = ?");
            params.push(serde_json::to_string(status)?);
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = query.limit {
            sql.push_str(" LIMIT ?");
            params.push(limit.to_string());
        }

        if let Some(offset) = query.offset {
            sql.push_str(" OFFSET ?");
            params.push(offset.to_string());
        }

        let mut query_builder = sqlx::query(&sql);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

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
    let created_at_str: String = row.get("created_at");

    let trigger_reason: DiagnosticTrigger = serde_json::from_str(&trigger_reason_json)?;
    let node_results: Vec<NodeTestResults> = serde_json::from_str(&node_results_json)?;
    let status: DiagnosticStatus = serde_json::from_str(&status_json)?;
    let started_at = chrono::DateTime::parse_from_rfc3339(&started_at_str)?.with_timezone(&chrono::Utc);
    let completed_at = completed_at_str
        .map(|s| chrono::DateTime::parse_from_rfc3339(&s))
        .transpose()?
        .map(|dt| dt.with_timezone(&chrono::Utc));
    let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&chrono::Utc);

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
        created_at,
    })
}