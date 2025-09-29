use crate::server::hosts::types::{
    base::{Host, HostBase},
    interfaces::Interface,
    ports::Port,
    targets::HostTarget,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait HostStorage: Send + Sync {
    async fn create(&self, host: &Host) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Host>>;
    async fn get_all(&self) -> Result<Vec<Host>>;
    async fn update(&self, host: &Host) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteHostStorage {
    pool: SqlitePool,
}

impl SqliteHostStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HostStorage for SqliteHostStorage {
    async fn create(&self, host: &Host) -> Result<()> {
        let services_str = serde_json::to_string(&host.base.services)?;
        let interfaces_str = serde_json::to_string(&host.base.interfaces)?;
        let target_str = serde_json::to_string(&host.base.target)?;
        let ports_str = serde_json::to_string(&host.base.ports)?;

        sqlx::query(
            r#"
            INSERT INTO hosts (
                id, name, hostname, target, description,
                services, interfaces, ports,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(blob_uuid::to_blob(&host.id))
        .bind(&host.base.name)
        .bind(&host.base.hostname)
        .bind(target_str)
        .bind(&host.base.description)
        .bind(services_str)
        .bind(interfaces_str)
        .bind(ports_str)
        .bind(&host.created_at.to_rfc3339())
        .bind(&host.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Host>> {
        let row = sqlx::query("SELECT * FROM hosts WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_host(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Host>> {
        let rows = sqlx::query("SELECT * FROM hosts ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut hosts = Vec::new();
        for row in rows {
            hosts.push(row_to_host(row)?);
        }

        Ok(hosts)
    }

    async fn update(&self, host: &Host) -> Result<()> {
        let services_str = serde_json::to_string(&host.base.services)?;
        let interfaces_str = serde_json::to_string(&host.base.interfaces)?;
        let target_str = serde_json::to_string(&host.base.target)?;
        let ports_str = serde_json::to_string(&host.base.ports)?;

        sqlx::query(
            r#"
            UPDATE hosts SET 
                name = ?, hostname = ?, description = ?,
                target = ?, interfaces = ?, ports = ?, services = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&host.base.name)
        .bind(&host.base.hostname)
        .bind(&host.base.description)
        .bind(target_str)
        .bind(interfaces_str)
        .bind(ports_str)
        .bind(services_str)
        .bind(&host.updated_at)
        .bind(blob_uuid::to_blob(&host.id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM hosts WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_host(row: sqlx::sqlite::SqliteRow) -> Result<Host, Error> {
    // Parse JSON fields safely
    let services: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("services"))
        .or(Err(Error::msg("Failed to deserialize services")))?;
    let interfaces: Vec<Interface> = serde_json::from_str(&row.get::<String, _>("interfaces"))
        .or(Err(Error::msg("Failed to deserialize interfaces")))?;
    let target: HostTarget = serde_json::from_str(&row.get::<String, _>("target"))
        .or(Err(Error::msg("Failed to deserialize target")))?;
    let ports: Vec<Port> = serde_json::from_str(&row.get::<String, _>("ports"))
        .or(Err(Error::msg("Failed to deserialize ports")))?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
        .with_timezone(&chrono::Utc);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?
        .with_timezone(&chrono::Utc);

    Ok(Host {
        id: blob_uuid::to_uuid(row.get("id")).or(Err(Error::msg("Failed to deserialize ID")))?,
        created_at,
        updated_at,
        base: HostBase {
            name: row.get("name"),
            target,
            hostname: row.get("hostname"),
            description: row.get("description"),
            services,
            ports,
            interfaces,
        },
    })
}
