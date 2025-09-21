use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

use crate::server::services::types::{base::{Service, ServiceBase}, ports::Port, types::ServiceType};

#[async_trait]
pub trait ServiceStorage: Send + Sync {
    async fn create(&self, service: &Service) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Service>>;
    async fn get_all(&self) -> Result<Vec<Service>>;
    async fn get_services_for_host(&self, host_id: &Uuid) -> Result<Vec<Service>>;
    async fn update(&self, service: &Service) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteServiceStorage {
    pool: SqlitePool,
}

impl SqliteServiceStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServiceStorage for SqliteServiceStorage {

    async fn create(&self, service: &Service) -> Result<()> {
        let service_type_str = serde_json::to_string(&service.base.service_type)?;
        let ports_str = serde_json::to_string(&service.base.ports)?;
        let interface_bindings_str = serde_json::to_string(&service.base.interface_bindings)?;
        let groups_str = serde_json::to_string(&service.base.groups)?;
        
        // Try to insert, ignore if constraint sviolation
        sqlx::query(
            r#"
            INSERT INTO services (
                id, name, host_id, service_type, ports, interface_bindings, groups, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&service.id)
        .bind(&service.base.name)
        .bind(&service.base.host_id)
        .bind(service_type_str)
        .bind(ports_str)
        .bind(interface_bindings_str)
        .bind(groups_str)
        .bind(&service.created_at.to_rfc3339())
        .bind(&service.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Service>> {
        let row = sqlx::query("SELECT * FROM services WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_service(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Service>> {
        let rows = sqlx::query("SELECT * FROM services ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut subnets = Vec::new();
        for row in rows {
            subnets.push(row_to_service(row)?);
        }

        Ok(subnets)
    }

    async fn get_services_for_host(&self, host_id: &Uuid) -> Result<Vec<Service>> {
        let rows = sqlx::query(
            "SELECT * FROM services WHERE host_id = ? ORDER BY created_at",
        )
        .bind(host_id)
        .fetch_all(&self.pool)
        .await?;

        let mut services = Vec::new();
        for row in rows {
            services.push(row_to_service(row)?);
        }

        Ok(services)
    }

    async fn update(&self, service: &Service) -> Result<()> {
        let service_type_str = serde_json::to_string(&service.base.service_type)?;
        let ports_str = serde_json::to_string(&service.base.ports)?;
        let interface_bindings_str = serde_json::to_string(&service.base.interface_bindings)?;
        let groups_str = serde_json::to_string(&service.base.groups)?;

        sqlx::query(
            r#"
            UPDATE services SET 
                name = ?, host_id = ?, service_type = ?, ports = ?, interface_bindings = ?, groups = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&service.base.name)
        .bind(&service.base.host_id)
        .bind(service_type_str)
        .bind(ports_str)
        .bind(interface_bindings_str)
        .bind(groups_str)
        .bind(&service.updated_at.to_rfc3339())
        .bind(&service.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM services WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_service(row: sqlx::sqlite::SqliteRow) -> Result<Service> {
    // Parse JSON fields safely
    let service_type: ServiceType = serde_json::from_str(&row.get::<String, _>("service_type"))?;
    let ports: Vec<Port> = serde_json::from_str(&row.get::<String, _>("ports"))?;
    let interface_bindings: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("interface_bindings"))?;
    let groups: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("groups"))?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
        .with_timezone(&chrono::Utc);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?
        .with_timezone(&chrono::Utc);

    Ok(Service {
        id: row.get("id"),
        created_at,
        updated_at,
        base: ServiceBase {
            name: row.get("name"),
            host_id: row.get("host_id"),
            service_type,
            ports,
            interface_bindings,
            groups
        }        
    })
}