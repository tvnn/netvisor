use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::server::{
    discovery::types::base::EntitySource,
    services::types::{
        base::{Service, ServiceBase},
        bindings::Binding,
        definitions::ServiceDefinition,
        virtualization::ServiceVirtualization,
    },
};

#[async_trait]
pub trait ServiceStorage: Send + Sync {
    async fn create(&self, service: &Service) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Service>>;
    async fn get_all(&self) -> Result<Vec<Service>>;
    async fn get_services_for_host(&self, host_id: &Uuid) -> Result<Vec<Service>>;
    async fn update(&self, service: &Service) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct PostgresServiceStorage {
    pool: PgPool,
}

impl PostgresServiceStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServiceStorage for PostgresServiceStorage {
    async fn create(&self, service: &Service) -> Result<()> {
        let service_def_str = serde_json::to_string(&service.base.service_definition)?;
        let bindings_str = serde_json::to_value(&service.base.bindings)?;
        let virtualization_str = serde_json::to_value(&service.base.virtualization)?;
        let vms_str = serde_json::to_value(&service.base.vms)?;
        let containers_str = serde_json::to_value(&service.base.containers)?;
        let source_str = serde_json::to_value(&service.base.source)?;

        sqlx::query(
            r#"
            INSERT INTO services (
                id, name, host_id, service_definition, bindings, virtualization, vms, containers, source, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&service.id)
        .bind(&service.base.name)
        .bind(&service.base.host_id)
        .bind(service_def_str)
        .bind(bindings_str)
        .bind(virtualization_str)
        .bind(vms_str)
        .bind(containers_str)
        .bind(source_str)
        .bind(service.created_at)
        .bind(service.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Service>> {
        let row = sqlx::query("SELECT * FROM services WHERE id = $1")
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

        let mut services = Vec::new();
        for row in rows {
            services.push(row_to_service(row)?);
        }

        Ok(services)
    }

    async fn get_services_for_host(&self, host_id: &Uuid) -> Result<Vec<Service>> {
        let rows = sqlx::query("SELECT * FROM services WHERE host_id = $1 ORDER BY created_at")
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
        let service_def_str = serde_json::to_string(&service.base.service_definition)?;
        let bindings_str = serde_json::to_value(&service.base.bindings)?;
        let virtualization_str = serde_json::to_value(&service.base.virtualization)?;
        let vms_str = serde_json::to_value(&service.base.vms)?;
        let containers_str = serde_json::to_value(&service.base.containers)?;
        let source_str = serde_json::to_value(&service.base.source)?;

        sqlx::query(
            r#"
            UPDATE services SET 
                name = $2, host_id = $3, service_definition = $4, bindings = $5, virtualization = $6, vms = $7, containers = $8, source = $9, updated_at = $10
            WHERE id = $1
            "#,
        )
        .bind(&service.id)
        .bind(&service.base.name)
        .bind(&service.base.host_id)
        .bind(service_def_str)
        .bind(bindings_str)
        .bind(virtualization_str)
        .bind(vms_str)
        .bind(containers_str)
        .bind(source_str)
        .bind(service.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM services WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_service(row: sqlx::postgres::PgRow) -> Result<Service, Error> {
    // Parse JSON fields safely
    let service_definition: Box<dyn ServiceDefinition> =
        serde_json::from_str(&row.get::<String, _>("service_definition"))
            .or(Err(Error::msg("Failed to deserialize service_definition")))?;
    let bindings: Vec<Binding> = serde_json::from_value(row.get::<serde_json::Value, _>("bindings"))
        .or(Err(Error::msg("Failed to deserialize bindings")))?;
    let virtualization: Option<ServiceVirtualization> =
        serde_json::from_value(row.get::<serde_json::Value, _>("virtualization"))
            .or(Err(Error::msg("Failed to deserialize virtualization")))?;
    let vms: Vec<Uuid> = serde_json::from_value(row.get::<serde_json::Value, _>("vms"))
        .or(Err(Error::msg("Failed to deserialize vms")))?;
    let containers: Vec<Uuid> = serde_json::from_value(row.get::<serde_json::Value, _>("containers"))
        .or(Err(Error::msg("Failed to deserialize containers")))?;
    let source: EntitySource = serde_json::from_value(row.get::<serde_json::Value, _>("source"))
        .or(Err(Error::msg("Failed to deserialize source")))?;

    Ok(Service {
        id: row.get("id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        base: ServiceBase {
            name: row.get("name"),
            host_id: row.get("host_id"),
            service_definition,
            virtualization,
            vms,
            containers,
            bindings,
            source,
        },
    })
}
