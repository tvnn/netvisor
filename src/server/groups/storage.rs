use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::groups::types::{GroupBase,Group};

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
        let services_json = serde_json::to_string(&group.base.services)?;

        sqlx::query(
            r#"
            INSERT INTO groups (
                id, name, description, services,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&group.id)
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(services_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Group>> {
        let row = sqlx::query("SELECT * FROM groups WHERE id = ?")
            .bind(id)
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
        let services_json = serde_json::to_string(&group.base.services)?;

        sqlx::query(
            r#"
            UPDATE groups SET 
                name = ?, description = ?, services = ?, 
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(services_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&group.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM groups WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_group(row: sqlx::sqlite::SqliteRow) -> Result<Group> {
    let services: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("services"))?;

    Ok(Group {
        id: row.get("id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        base: GroupBase {
            name: row.get("name"),
            description: row.get("description"),
            services,
        }  
    })
}