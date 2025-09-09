use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::host_groups::types::{HostGroupBase,HostGroup};

#[async_trait]
pub trait HostGroupStorage: Send + Sync {
    async fn create(&self, group: &HostGroup) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<HostGroup>>;
    async fn get_all(&self) -> Result<Vec<HostGroup>>;
    async fn update(&self, group: &HostGroup) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteHostGroupStorage {
    pool: SqlitePool,
}

impl SqliteHostGroupStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HostGroupStorage for SqliteHostGroupStorage {
    async fn create(&self, group: &HostGroup) -> Result<()> {
        let hosts_json = serde_json::to_string(&group.base.hosts)?;

        sqlx::query(
            r#"
            INSERT INTO host_groups (
                id, name, description, hosts
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&group.id)
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(hosts_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<HostGroup>> {
        let row = sqlx::query("SELECT * FROM host_groups WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_host_group(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<HostGroup>> {
        let rows = sqlx::query("SELECT * FROM host_groups ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut groups = Vec::new();
        for row in rows {
            groups.push(row_to_host_group(row)?);
        }

        Ok(groups)
    }

    async fn update(&self, group: &HostGroup) -> Result<()> {
        let hosts_json = serde_json::to_string(&group.base.hosts)?;

        sqlx::query(
            r#"
            UPDATE host_groups SET 
                name = ?, description = ?, hosts = ?, 
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&group.base.name)
        .bind(&group.base.description)
        .bind(hosts_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&group.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM host_groups WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_host_group(row: sqlx::sqlite::SqliteRow) -> Result<HostGroup> {
    let hosts: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("hosts"))?;

    Ok(HostGroup {
        id: row.get("id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        base: HostGroupBase {
            name: row.get("name"),
            description: row.get("description"),
            hosts,
        }  
    })
}