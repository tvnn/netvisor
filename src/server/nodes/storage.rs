use async_trait::async_trait;
use anyhow::Result;
use cidr::IpCidr;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::server::nodes::types::{base::{DiscoveryStatus, Node, NodeBase}, capabilities::NodeCapability, status::NodeStatus, targets::NodeTarget, tests::AssignedTest, types::NodeType};

#[async_trait]
pub trait NodeStorage: Send + Sync {
    async fn create(&self, node: &Node) -> Result<()>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Node>>;
    async fn get_all(&self) -> Result<Vec<Node>>;
    async fn update(&self, node: &Node) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
    async fn get_by_group(&self, group_id: &Uuid) -> Result<Vec<Node>>;
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
        let capabilities_str = serde_json::to_string(&node.base.capabilities)?;
        let assigned_tests_str = serde_json::to_string(&node.base.assigned_tests)?;
        let node_groups_str = serde_json::to_string(&node.base.node_groups)?;
        let subnets_str = serde_json::to_string(&node.base.subnets)?;
        let node_type_str = serde_json::to_string(&node.base.node_type)?;
        let last_seen_str = node.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_str = serde_json::to_string(&node.base.target)?;
        let status_str = serde_json::to_string(&node.base.status)?;
        let discovery_status_str = serde_json::to_string(&node.base.discovery_status)?;

        sqlx::query(
            r#"
            INSERT INTO nodes (
                id, name, hostname, target, description,
                node_type, capabilities, assigned_tests, monitoring_interval,
                node_groups, status, discovery_status, subnets,
                mac_address, last_seen, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&node.id)
        .bind(&node.base.name)
        .bind(node_type_str)
        .bind(&node.base.hostname)
        .bind(&node.base.mac_address)
        .bind(&node.base.description)
        .bind(target_str)
        .bind(subnets_str)
        .bind(discovery_status_str)
        .bind(capabilities_str)
        .bind(status_str)
        .bind(assigned_tests_str)
        .bind(node.base.monitoring_interval)
        .bind(node_groups_str)
        .bind(last_seen_str)
        .bind(&node.created_at)
        .bind(&node.updated_at)
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
        let capabilities_str = serde_json::to_string(&node.base.capabilities)?;
        let assigned_tests_str = serde_json::to_string(&node.base.assigned_tests)?;
        let node_groups_str = serde_json::to_string(&node.base.node_groups)?;
        let subnets_str = serde_json::to_string(&node.base.subnets)?;
        let node_type_str = serde_json::to_string(&node.base.node_type)?;
        let last_seen_str = node.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_str = serde_json::to_string(&node.base.target)?;
        let status_str = serde_json::to_string(&node.base.status)?;
        let discovery_status_str = serde_json::to_string(&node.base.discovery_status)?;

        sqlx::query(
            r#"
            UPDATE nodes SET 
                name = ?, hostname = ?, target = ?, description = ?,
                node_type = ?, capabilities = ?, assigned_tests = ?, monitoring_interval = ?,
                node_groups = ?, status = ?, discovery_status = ?, subnets = ?,
                mac_address = ?, last_seen = ?, created_at = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&node.base.name)
        .bind(node_type_str)
        .bind(&node.base.hostname)
        .bind(&node.base.mac_address)
        .bind(&node.base.description)
        .bind(target_str)
        .bind(subnets_str)
        .bind(discovery_status_str)
        .bind(capabilities_str)
        .bind(status_str)
        .bind(assigned_tests_str)
        .bind(node.base.monitoring_interval)
        .bind(node_groups_str)
        .bind(last_seen_str)
        .bind(&node.created_at)
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

    async fn get_by_group(&self, group_id: &Uuid) -> Result<Vec<Node>> {
        let rows = sqlx::query("SELECT * FROM nodes WHERE JSON_EXTRACT(node_groups, '$') LIKE ?")
            .bind(format!("%\"{}\"$", group_id))
            .fetch_all(&self.pool)
            .await?;

        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row_to_node(row)?);
        }

        Ok(nodes)
    }
}

fn row_to_node(row: sqlx::sqlite::SqliteRow) -> Result<Node> {
    let capabilities: Vec<NodeCapability> = serde_json::from_str(row.get("capabilities"))?;
    let hostname: String = serde_json::from_str(row.get("hostname"))?;
    let assigned_tests: Vec<AssignedTest> = serde_json::from_str(row.get("assigned_tests"))?;
    let node_groups: Vec<Uuid> = serde_json::from_str(row.get("node_groups"))?;
    let subnets: Vec<IpCidr> = serde_json::from_str(row.get("subnets"))?;
    let status: NodeStatus = serde_json::from_str(row.get("status"))?;
    let target: NodeTarget = serde_json::from_str(row.get("target"))?;
    let node_type: NodeType = serde_json::from_str(row.get("node_type"))?;
    let discovery_status: DiscoveryStatus = serde_json::from_str(row.get("discovery_status"))?;
        
    let last_seen = match row.get::<Option<String>, _>("last_seen") {
        Some(dt_str) => Some(chrono::DateTime::parse_from_rfc3339(&dt_str)?.with_timezone(&chrono::Utc)),
        None => None,
    };

    Ok(Node {
        id: row.get("id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        last_seen,
        base: NodeBase {
            name: row.get("name"),
            target,
            hostname: Some(hostname),
            description: row.get("description"),
            node_type,
            capabilities,
            discovery_status: Some(discovery_status),
            assigned_tests,
            mac_address: row.get("mac_address"),
            monitoring_interval: row.get("monitoring_interval"),
            node_groups,
            status,
            subnets,
        }        
    })
}