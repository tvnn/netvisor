use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::hosts::types::base::HostSubnetMembership;
use crate::server::{hosts::types::{base::{Host, HostBase}, targets::HostTarget}, services::types::{base::Service, ports::Port}};

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
        let groups_str = serde_json::to_string(&host.base.groups)?;
        let subnets_str = serde_json::to_string(&host.base.subnets)?;
        let last_seen_str = host.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_str = serde_json::to_string(&host.base.target)?;
        let open_ports_str = serde_json::to_string(&host.base.open_ports)?;

        sqlx::query(
            r#"
            INSERT INTO hosts (
                id, name, hostname, target, description,
                services, groups, subnets, open_ports,
                last_seen, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&host.id)
        .bind(&host.base.name)
        .bind(&host.base.hostname)
        .bind(target_str)
        .bind(&host.base.description)
        .bind(services_str)
        .bind(groups_str)
        .bind(subnets_str)
        .bind(open_ports_str)
        .bind(last_seen_str)
        .bind(&host.created_at.to_rfc3339())
        .bind(&host.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Host>> {
        let row = sqlx::query("SELECT * FROM hosts WHERE id = ?")
            .bind(id)
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
        let groups_str = serde_json::to_string(&host.base.groups)?;
        let subnets_str = serde_json::to_string(&host.base.subnets)?;
        let last_seen_str = host.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_str = serde_json::to_string(&host.base.target)?;
        let open_ports_str = serde_json::to_string(&host.base.open_ports)?;

        sqlx::query(
            r#"
            UPDATE hosts SET 
                name = ?, hostname = ?, description = ?,
                target = ?, subnets = ?, open_ports = ?, services = ?, groups = ?,
                last_seen = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&host.base.name)
        .bind(&host.base.hostname)
        .bind(&host.base.description)
        .bind(target_str)
        .bind(subnets_str)
        .bind(open_ports_str)
        .bind(services_str)
        .bind(groups_str)
        .bind(last_seen_str)
        .bind(&host.updated_at)
        .bind(&host.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM hosts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_host(row: sqlx::sqlite::SqliteRow) -> Result<Host> {
    // Parse JSON fields safely
    let services: Vec<Service> = serde_json::from_str(&row.get::<String, _>("services"))?;
    let groups: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("groups"))?;
    let subnets: Vec<HostSubnetMembership> = serde_json::from_str(&row.get::<String, _>("subnets"))?;
    let target: HostTarget = serde_json::from_str(&row.get::<String, _>("target"))?;
    let open_ports: Vec<Port> = serde_json::from_str(&row.get::<String, _>("open_ports"))?;
    
    // Handle datetime fields  
    let last_seen = match row.get::<Option<String>, _>("last_seen") {
        Some(dt_str) => Some(chrono::DateTime::parse_from_rfc3339(&dt_str)?.with_timezone(&chrono::Utc)),
        None => None,
    };

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
        .with_timezone(&chrono::Utc);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?
        .with_timezone(&chrono::Utc);

    Ok(Host {
        id: row.get("id"),
        created_at,
        updated_at,
        last_seen,
        base: HostBase {
            name: row.get("name"),
            target,
            hostname: row.get("hostname"),
            description: row.get("description"),
            services,
            open_ports,
            groups,
            subnets,
        }        
    })
}