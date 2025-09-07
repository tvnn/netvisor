use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::{nodes::types::{base::{DiscoveryStatus, Node, NodeBase}, targets::NodeTarget}, services::types::base::Service, subnets::types::base::NodeSubnetMembership};

#[async_trait]
pub trait NodeStorage: Send + Sync {
    async fn create(&self, node: &Node) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Node>>;
    async fn get_all(&self) -> Result<Vec<Node>>;
    async fn update(&self, node: &Node) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteNodeStorage {
    pool: SqlitePool,
}

impl SqliteNodeStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NodeStorage for SqliteNodeStorage {
    async fn create(&self, node: &Node) -> Result<()> {
        let services_str = serde_json::to_string(&node.base.services)?;
        let node_groups_str = serde_json::to_string(&node.base.node_groups)?;
        let subnets_str = serde_json::to_string(&node.base.subnets)?;
        let last_seen_str = node.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_str = serde_json::to_string(&node.base.target)?;
        let discovery_status_str = match &node.base.discovery_status {
            Some(status) => serde_json::to_string(status)?,
            None => "null".to_string(),
        };

        sqlx::query(
            r#"
            INSERT INTO nodes (
                id, name, hostname, target, description,
                services, node_groups, discovery_status, subnets,
                last_seen, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&node.id)
        .bind(&node.base.name)
        .bind(&node.base.hostname)
        .bind(target_str)
        .bind(&node.base.description)
        .bind(services_str)
        .bind(node_groups_str)
        .bind(discovery_status_str)
        .bind(subnets_str)
        .bind(last_seen_str)
        .bind(&node.created_at.to_rfc3339())
        .bind(&node.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Node>> {
        let row = sqlx::query("SELECT * FROM nodes WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(row_to_node(row)?)),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Node>> {
        let rows = sqlx::query("SELECT * FROM nodes ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row_to_node(row)?);
        }

        Ok(nodes)
    }

    async fn update(&self, node: &Node) -> Result<()> {
        let services_str = serde_json::to_string(&node.base.services)?;
        let node_groups_str = serde_json::to_string(&node.base.node_groups)?;
        let subnets_str = serde_json::to_string(&node.base.subnets)?;
        let last_seen_str = node.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_str = serde_json::to_string(&node.base.target)?;
        let discovery_status_str = serde_json::to_string(&node.base.discovery_status)?;

        sqlx::query(
            r#"
            UPDATE nodes SET 
                name = ?, hostname = ?, description = ?,
                target = ?, subnets = ?, discovery_status = ?, services = ?, node_groups = ?,
                last_seen = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&node.base.name)
        .bind(&node.base.hostname)
        .bind(&node.base.description)
        .bind(target_str)
        .bind(subnets_str)
        .bind(discovery_status_str)
        .bind(services_str)
        .bind(node_groups_str)
        .bind(last_seen_str)
        .bind(&node.updated_at)
        .bind(&node.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM nodes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn row_to_node(row: sqlx::sqlite::SqliteRow) -> Result<Node> {
    // Parse JSON fields safely
    let services: Vec<Service> = serde_json::from_str(&row.get::<String, _>("services"))?;
    let node_groups: Vec<Uuid> = serde_json::from_str(&row.get::<String, _>("node_groups"))?;
    let subnets: Vec<NodeSubnetMembership> = serde_json::from_str(&row.get::<String, _>("subnets"))?;
    let target: NodeTarget = serde_json::from_str(&row.get::<String, _>("target"))?;
    
    // Handle nullable discovery_status
    let discovery_status: Option<DiscoveryStatus> = {
        let discovery_str: String = row.get("discovery_status");
        if discovery_str == "null" {
            None
        } else {
            Some(serde_json::from_str(&discovery_str)?)
        }
    };

    // Handle datetime fields  
    let last_seen = match row.get::<Option<String>, _>("last_seen") {
        Some(dt_str) => Some(chrono::DateTime::parse_from_rfc3339(&dt_str)?.with_timezone(&chrono::Utc)),
        None => None,
    };

    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
        .with_timezone(&chrono::Utc);
    let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?
        .with_timezone(&chrono::Utc);

    Ok(Node {
        id: row.get("id"),
        created_at,
        updated_at,
        last_seen,
        base: NodeBase {
            name: row.get("name"),
            target,
            hostname: row.get("hostname"),
            description: row.get("description"),
            services,
            discovery_status,
            node_groups,
            subnets,
        }        
    })
}