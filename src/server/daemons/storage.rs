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

        sqlx::query(
            r#"
            INSERT INTO daemons (
                id, node_id,
                last_seen, registered_at
            ) VALUES (?, ?, ?, ?)
            "#
        )
        .bind(&daemon.id)
        .bind(&daemon.base.node_id)
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
        let rows = sqlx::query("SELECT * FROM daemons")
            .fetch_all(&self.pool)
            .await?;

        let mut groups = Vec::new();
        for row in rows {
            groups.push(row_to_daemon(row)?);
        }

        Ok(groups)
    }

    async fn update(&self, daemon: &Daemon) -> Result<()> {

        sqlx::query(
            r#"
            UPDATE daemons SET 
                node_id = ?, last_seen = ?
            WHERE id = ?
            "#
        )
        .bind(&daemon.base.node_id)
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
    Ok(Daemon {
        id: row.get("id"),
        last_seen: row.get("last_seen"),
        registered_at: row.get("registered_at"),
        base: DaemonBase {
            node_id: row.get("node_id"),
        }
    })
}