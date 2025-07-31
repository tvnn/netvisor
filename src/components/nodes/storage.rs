use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::shared::storage::{StorageResult,StorageError};
use super::types::{NetworkNode};

#[async_trait]
pub trait NodeStorage: Send + Sync {
    async fn get_nodes(&self) -> StorageResult<Vec<NetworkNode>>;
    async fn get_node(&self, id: &str) -> StorageResult<NetworkNode>;
    async fn save_node(&self, node: &NetworkNode) -> StorageResult<()>;
    async fn update_node(&self, id: &str, node: &NetworkNode) -> StorageResult<()>;
    async fn delete_node(&self, id: &str) -> StorageResult<()>;
}

pub struct SqliteNodeStorage {
    pool: SqlitePool,
}

impl SqliteNodeStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NodeStorage for SqliteNodeStorage {
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
}