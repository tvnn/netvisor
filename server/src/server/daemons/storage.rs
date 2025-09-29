use std::net::IpAddr;

use crate::server::daemons::types::base::{Daemon, DaemonBase};
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait DaemonStorage: Send + Sync {
    async fn create(&self, daemon: &Daemon) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Daemon>>;
    async fn get_all(&self) -> Result<Vec<Daemon>>;
    async fn update(&self, group: &Daemon) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteDaemonStorage {
    pool: SqlitePool,
}

impl SqliteDaemonStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DaemonStorage for SqliteDaemonStorage {
    async fn create(&self, daemon: &Daemon) -> Result<()> {
        let ip_str = serde_json::to_string(&daemon.base.ip)?;

        sqlx::query(
            r#"
            INSERT INTO daemons (
                id, host_id, ip, port,
                last_seen, registered_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(blob_uuid::to_blob(&daemon.id))
        .bind(blob_uuid::to_blob(&daemon.base.host_id))
        .bind(ip_str)
        .bind(&daemon.base.port)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Daemon>> {
        let row = sqlx::query("SELECT * FROM daemons WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_daemon(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Daemon>> {
        let rows = sqlx::query("SELECT * FROM daemons")
            .fetch_all(&self.pool)
            .await?;

        let mut daemons = Vec::new();
        for row in rows {
            daemons.push(row_to_daemon(row)?);
        }

        Ok(daemons)
    }

    async fn update(&self, daemon: &Daemon) -> Result<()> {
        let ip_str = serde_json::to_string(&daemon.base.ip)?;

        sqlx::query(
            r#"
            UPDATE daemons SET 
                host_id = ?, ip = ?, port = ?, last_seen = ?
            WHERE id = ?
            "#,
        )
        .bind(blob_uuid::to_blob(&daemon.base.host_id))
        .bind(ip_str)
        .bind(&daemon.base.port)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(blob_uuid::to_blob(&daemon.id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM daemons WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_daemon(row: sqlx::sqlite::SqliteRow) -> Result<Daemon, Error> {
    let ip: IpAddr = serde_json::from_str(&row.get::<String, _>("ip"))
        .or(Err(Error::msg("Failed to deserialize IP")))?;

    Ok(Daemon {
        id: blob_uuid::to_uuid(row.get("id")).or(Err(Error::msg("Failed to deserialize ID")))?,
        last_seen: row.get("last_seen"),
        registered_at: row.get("registered_at"),
        base: DaemonBase {
            ip,
            port: row.get("port"),
            host_id: blob_uuid::to_uuid(row.get("host_id"))
                .or(Err(Error::msg("Failed to deserialize host_id")))?,
        },
    })
}
