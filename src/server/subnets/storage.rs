use async_trait::async_trait;
use anyhow::Result;
use cidr::IpCidr;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::subnets::types::base::{Subnet, SubnetBase, SubnetSource, SubnetType};

#[async_trait]
pub trait SubnetStorage: Send + Sync {
    async fn create(&self, subnet: &Subnet) -> Result<Subnet>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Subnet>>;
    async fn get_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Subnet>>;
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

    async fn create(&self, subnet: &Subnet) -> Result<Subnet> {
        let cidr_str = serde_json::to_string(&subnet.base.cidr)?;
        let gateways_str = serde_json::to_string(&subnet.base.gateways)?;
        let dns_resolvers_str = serde_json::to_string(&subnet.base.dns_resolvers)?;
        let subnet_type_str = serde_json::to_string(&subnet.base.subnet_type)?;
        let subnet_source_str = serde_json::to_string(&subnet.base.source)?;

        // Try to insert, ignore if constraint sviolation
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO subnets (
                id, name, description, cidr, dns_resolvers, gateways, subnet_type, source, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&subnet.id)
        .bind(&subnet.base.name)
        .bind(&subnet.base.description)
        .bind(&cidr_str)
        .bind(dns_resolvers_str)
        .bind(&gateways_str)
        .bind(subnet_type_str)
        .bind(subnet_source_str)
        .bind(&subnet.created_at.to_rfc3339())
        .bind(&subnet.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        // Always query for the actual record (either just inserted or existing)
        let existing = sqlx::query("SELECT * FROM subnets WHERE cidr = ? AND gateways = ?")
            .bind(&cidr_str)
            .bind(&gateways_str)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_subnet(existing)?)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Subnet>> {
        let row = sqlx::query("SELECT * FROM subnets WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_subnet(row)?)),
            None => Ok(None),
        }
    }

    async fn get_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Subnet>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!("SELECT * FROM subnets WHERE id IN ({})", placeholders);
        
        let mut query_builder = sqlx::query(&query);
        for id in ids {
            query_builder = query_builder.bind(id);
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
        let dns_resolvers_str = serde_json::to_string(&subnet.base.dns_resolvers)?;
        let gateways_str = serde_json::to_string(&subnet.base.gateways)?;
        let subnet_type_str = serde_json::to_string(&subnet.base.subnet_type)?;
        let subnet_source_str = serde_json::to_string(&subnet.base.source)?;

        sqlx::query(
            r#"
            UPDATE subnets SET 
                name = ?, description = ?, cidr = ?, dns_resolvers = ?, gateways = ?, subnet_type = ?, source = ?,
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&subnet.base.name)
        .bind(&subnet.base.description)
        .bind(cidr_str)
        .bind(dns_resolvers_str)
        .bind(gateways_str)
        .bind(subnet_type_str)
        .bind(subnet_source_str)
        .bind(&subnet.updated_at.to_rfc3339())
        .bind(&subnet.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM subnets WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_subnet(row: sqlx::sqlite::SqliteRow) -> Result<Subnet> {
    // Parse JSON fields safely
    let cidr: IpCidr = serde_json::from_str(&row.get::<String, _>("cidr"))?;
    let dns_resolvers: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("dns_resolvers"))?;
    let gateways: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("gateways"))?;
    let subnet_type: SubnetType = serde_json::from_str(&row.get::<String, _>("subnet_type"))?;
    let source: SubnetSource = serde_json::from_str(&row.get::<String, _>("source"))?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
        .with_timezone(&chrono::Utc);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?
        .with_timezone(&chrono::Utc);

    Ok(Subnet {
        id: row.get("id"),
        created_at,
        updated_at,
        base: SubnetBase {
            name: row.get("name"),
            description: row.get("description"),
            source,
            cidr,
            dns_resolvers,
            gateways,
            subnet_type,
        }        
    })
}