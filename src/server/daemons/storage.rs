use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::daemons::types::base::{Daemon, DaemonBase};

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

        let ip_json = serde_json::to_string(&daemon.base.ip)?;
        let status_json = serde_json::to_string(&daemon.base.status)?;

        sqlx::query(
            r#"
            INSERT INTO daemons (
                id, name, ip, port, hostname, status,
                last_seen, registered_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&daemon.id)
        .bind(&daemon.base.name)
        .bind(ip_json)
        .bind(&daemon.base.port)
        .bind(&daemon.base.hostname)
        .bind(status_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Daemon>> {
        let row = sqlx::query("SELECT * FROM daemons WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_daemon(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Daemon>> {
        let rows = sqlx::query("SELECT * FROM daemons ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut groups = Vec::new();
        for row in rows {
            groups.push(row_to_daemon(row)?);
        }

        Ok(groups)
    }

    async fn update(&self, daemon: &Daemon) -> Result<()> {
        let ip_json = serde_json::to_string(&daemon.base.ip)?;
        let status_json = serde_json::to_string(&daemon.base.status)?;

        sqlx::query(
            r#"
            UPDATE daemons SET 
                name = ?, ip = ?, port = ?, hostname = ?, 
                status = ?, last_seen = ?
            WHERE id = ?
            "#
        )
        .bind(&daemon.base.name)
        .bind(ip_json)
        .bind(&daemon.base.port)
        .bind(&daemon.base.hostname)
        .bind(status_json)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM daemons WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_daemon(row: sqlx::sqlite::SqliteRow) -> Result<Daemon> {
    let ip_json: String = row.get("ip");
    let status_json: String = row.get("status");
    
    let ip = serde_json::from_str(&ip_json)?;
    let status = serde_json::from_str(&status_json)?;

    Ok(Daemon {
        id: row.get("id"),
        last_seen: row.get("last_seen"),
        registered_at: row.get("registered_at"),
        base: DaemonBase {
            ip,
            name: row.get("name"),
            port: row.get("port"),
            hostname: row.get("hostname"),
            status,
        }
    })
}