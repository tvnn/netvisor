use crate::server::{
    discovery::types::base::EntitySource,
    groups::types::{Group, GroupBase, GroupType},
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait GroupStorage: Send + Sync {
    async fn create(&self, group: &Group) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Group>>;
    async fn get_all(&self) -> Result<Vec<Group>>;
    async fn update(&self, group: &Group) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteGroupStorage {
    pool: SqlitePool,
}

impl SqliteGroupStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GroupStorage for SqliteGroupStorage {
    async fn create(&self, group: &Group) -> Result<()> {
        let services_json = serde_json::to_string(&group.base.service_bindings)?;
        let group_type_json = serde_json::to_string(&group.base.group_type)?;
        let source_json = serde_json::to_string(&group.base.source)?;

        sqlx::query(
            r#"
            INSERT INTO groups (
                id, name, description, service_bindings, group_type, source,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(blob_uuid::to_blob(&group.id))
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(services_json)
        .bind(group_type_json)
        .bind(source_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Group>> {
        let row = sqlx::query("SELECT * FROM groups WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_group(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Group>> {
        let rows = sqlx::query("SELECT * FROM groups ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut groups = Vec::new();
        for row in rows {
            groups.push(row_to_group(row)?);
        }

        Ok(groups)
    }

    async fn update(&self, group: &Group) -> Result<()> {
        let services_json = serde_json::to_string(&group.base.service_bindings)?;
        let group_type_json = serde_json::to_string(&group.base.group_type)?;
        let source_json = serde_json::to_string(&group.base.source)?;

        sqlx::query(
            r#"
            UPDATE groups SET 
                name = ?, description = ?, service_bindings = ?, group_type = ?, source = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(services_json)
        .bind(group_type_json)
        .bind(source_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(blob_uuid::to_blob(&group.id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM groups WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_group(row: sqlx::sqlite::SqliteRow) -> Result<Group, Error> {
    let service_bindings: Vec<Uuid> =
        serde_json::from_str(&row.get::<String, _>("service_bindings"))
            .or(Err(Error::msg("Failed to deserialize service bindings")))?;
    let group_type: GroupType = serde_json::from_str(&row.get::<String, _>("group_type"))
        .or(Err(Error::msg("Failed to deserialize group_type")))?;

    let source: EntitySource = serde_json::from_str(&row.get::<String, _>("source"))
        .or(Err(Error::msg("Failed to deserialize group_type")))?;

    Ok(Group {
        id: blob_uuid::to_uuid(row.get("id")).or(Err(Error::msg("Failed to deserialize ID")))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        base: GroupBase {
            name: row.get("name"),
            description: row.get("description"),
            service_bindings,
            source,
            group_type,
        },
    })
}
