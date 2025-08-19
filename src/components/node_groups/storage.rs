use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use crate::components::node_groups::types::{NodeGroupBase,NodeGroup};

#[async_trait]
pub trait NodeGroupStorage: Send + Sync {
    async fn create(&self, group: &NodeGroup) -> Result<()>;
    async fn get_by_id(&self, id: &str) -> Result<Option<NodeGroup>>;
    async fn get_all(&self) -> Result<Vec<NodeGroup>>;
    async fn update(&self, group: &NodeGroup) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
}

pub struct SqliteNodeGroupStorage {
    pool: SqlitePool,
}

impl SqliteNodeGroupStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NodeGroupStorage for SqliteNodeGroupStorage {
    async fn create(&self, group: &NodeGroup) -> Result<()> {
        let node_sequence_json = serde_json::to_string(&group.base.node_sequence)?;

        sqlx::query(
            r#"
            INSERT INTO node_groups (
                id, name, description, node_sequence, auto_diagnostic_enabled,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&group.id)
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(node_sequence_json)
        .bind(&group.base.auto_diagnostic_enabled)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<NodeGroup>> {
        let row = sqlx::query("SELECT * FROM node_groups WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_node_group(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<NodeGroup>> {
        let rows = sqlx::query("SELECT * FROM node_groups ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut groups = Vec::new();
        for row in rows {
            groups.push(row_to_node_group(row)?);
        }

        Ok(groups)
    }

    async fn update(&self, group: &NodeGroup) -> Result<()> {
        let node_sequence_json = serde_json::to_string(&group.base.node_sequence)?;

        sqlx::query(
            r#"
            UPDATE node_groups SET 
                name = ?, description = ?, node_sequence = ?, 
                auto_diagnostic_enabled = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(node_sequence_json)
        .bind(&group.base.auto_diagnostic_enabled)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&group.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM node_groups WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_node_group(row: sqlx::sqlite::SqliteRow) -> Result<NodeGroup> {
    let node_sequence_json: String = row.get("node_sequence");
    let node_sequence = serde_json::from_str(&node_sequence_json)?;

    Ok(NodeGroup {
        id: row.get("id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        base: NodeGroupBase {
            name: row.get("name"),
            description: row.get("description"),
            node_sequence,
            auto_diagnostic_enabled: row.get("auto_diagnostic_enabled"),
        }  
    })
}