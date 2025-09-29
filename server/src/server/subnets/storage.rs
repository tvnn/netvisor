use crate::server::subnets::types::base::{Subnet, SubnetBase, SubnetSource, SubnetType};
use anyhow::{Error, Result};
use async_trait::async_trait;
use cidr::IpCidr;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait SubnetStorage: Send + Sync {
    async fn create(&self, subnet: &Subnet) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Subnet>>;
    async fn get_by_ids(&self, ids: &Vec<Uuid>) -> Result<Vec<Subnet>>;
    async fn get_all(&self) -> Result<Vec<Subnet>>;
    async fn update(&self, subnet: &Subnet) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteSubnetStorage {
    pool: SqlitePool,
}

impl SqliteSubnetStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubnetStorage for SqliteSubnetStorage {
    async fn create(&self, subnet: &Subnet) -> Result<()> {
        let cidr_str = serde_json::to_string(&subnet.base.cidr)?;
        let gateways_str = serde_json::to_string(&subnet.base.gateways)?;
        let dns_resolvers_str = serde_json::to_string(&subnet.base.dns_resolvers)?;
        let reverse_proxies_str = serde_json::to_string(&subnet.base.reverse_proxies)?;
        let hosts_str = serde_json::to_string(&subnet.base.hosts)?;
        let subnet_type_str = serde_json::to_string(&subnet.base.subnet_type)?;
        let subnet_source_str = serde_json::to_string(&subnet.base.source)?;

        sqlx::query(
            r#"
            INSERT INTO subnets (
                id, name, description, cidr, hosts, dns_resolvers, gateways, 
                reverse_proxies, subnet_type, source, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(blob_uuid::to_blob(&subnet.id))
        .bind(&subnet.base.name)
        .bind(&subnet.base.description)
        .bind(&cidr_str)
        .bind(hosts_str)
        .bind(dns_resolvers_str)
        .bind(gateways_str)
        .bind(reverse_proxies_str)
        .bind(subnet_type_str)
        .bind(subnet_source_str)
        .bind(&subnet.created_at.to_rfc3339())
        .bind(&subnet.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Subnet>> {
        let row = sqlx::query("SELECT * FROM subnets WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_subnet(row)?)),
            None => Ok(None),
        }
    }

    async fn get_by_ids(&self, ids: &Vec<Uuid>) -> Result<Vec<Subnet>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!("SELECT * FROM subnets WHERE id IN ({})", placeholders);

        let mut query_builder = sqlx::query(&query);
        for id in ids {
            query_builder = query_builder.bind(blob_uuid::to_blob(id));
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        rows.into_iter()
            .map(|row| row_to_subnet(row))
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_all(&self) -> Result<Vec<Subnet>> {
        let rows = sqlx::query("SELECT * FROM subnets ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut subnets = Vec::new();
        for row in rows {
            subnets.push(row_to_subnet(row)?);
        }

        Ok(subnets)
    }

    async fn update(&self, subnet: &Subnet) -> Result<()> {
        let cidr_str = serde_json::to_string(&subnet.base.cidr)?;
        let hosts_str = serde_json::to_string(&subnet.base.hosts)?;
        let dns_resolvers_str = serde_json::to_string(&subnet.base.dns_resolvers)?;
        let gateways_str = serde_json::to_string(&subnet.base.gateways)?;
        let reverse_proxies_str = serde_json::to_string(&subnet.base.reverse_proxies)?;
        let subnet_type_str = serde_json::to_string(&subnet.base.subnet_type)?;
        let subnet_source_str = serde_json::to_string(&subnet.base.source)?;

        sqlx::query(
            r#"
            UPDATE subnets SET 
                name = ?, description = ?, cidr = ?, hosts = ?, dns_resolvers = ?, gateways = ?,
                reverse_proxies = ?, subnet_type = ?, source = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&subnet.base.name)
        .bind(&subnet.base.description)
        .bind(cidr_str)
        .bind(hosts_str)
        .bind(dns_resolvers_str)
        .bind(gateways_str)
        .bind(reverse_proxies_str)
        .bind(subnet_type_str)
        .bind(subnet_source_str)
        .bind(&subnet.updated_at.to_rfc3339())
        .bind(blob_uuid::to_blob(&subnet.id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM subnets WHERE id = ?")
            .bind(blob_uuid::to_blob(id))
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_subnet(row: sqlx::sqlite::SqliteRow) -> Result<Subnet, Error> {
    // Parse JSON fields safely
    let cidr: IpCidr = serde_json::from_str(&row.get::<String, _>("cidr"))
        .or(Err(Error::msg("Failed to deserialize cidr")))?;
    let hosts: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("hosts"))
        .or(Err(Error::msg("Failed to deserialize hosts")))?;
    let dns_resolvers: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("dns_resolvers"))
        .or(Err(Error::msg("Failed to deserialize dns_resolvers")))?;
    let gateways: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("gateways"))
        .or(Err(Error::msg("Failed to deserialize gateways")))?;
    let reverse_proxies: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("reverse_proxies"))
        .or(Err(Error::msg("Failed to deserialize reverse_proxies")))?;
    let subnet_type: SubnetType = serde_json::from_str(&row.get::<String, _>("subnet_type"))
        .or(Err(Error::msg("Failed to deserialize subnet_type")))?;
    let source: SubnetSource = serde_json::from_str(&row.get::<String, _>("source"))
        .or(Err(Error::msg("Failed to deserialize source")))?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
        .with_timezone(&chrono::Utc);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?
        .with_timezone(&chrono::Utc);

    Ok(Subnet {
        id: blob_uuid::to_uuid(row.get("id")).or(Err(Error::msg("Failed to deserialize id")))?,
        created_at,
        updated_at,
        base: SubnetBase {
            name: row.get("name"),
            description: row.get("description"),
            source,
            cidr,
            dns_resolvers,
            reverse_proxies,
            hosts,
            gateways,
            subnet_type,
        },
    })
}
