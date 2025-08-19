use async_trait::async_trait;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use crate::components::nodes::types::{
    base::{DetectedService, Node, NodeBase},
    targets::NodeTarget, 
    tests::{AssignedTest, NodeStatus},
    topology::GraphPosition, types_capabilities::{NodeCapability, NodeType},
};

#[async_trait]
pub trait NodeStorage: Send + Sync {
    async fn create(&self, node: &Node) -> Result<()>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Node>>;
    async fn get_all(&self) -> Result<Vec<Node>>;
    async fn update(&self, node: &Node) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn get_by_group(&self, group_id: &str) -> Result<Vec<Node>>;
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
        let capabilities_json = serde_json::to_string(&node.base.capabilities)?;
        let assigned_tests_json = serde_json::to_string(&node.base.assigned_tests)?;
        let node_groups_json = serde_json::to_string(&node.base.node_groups)?;
        let position_json = node.base.position.as_ref().map(|p| serde_json::to_string(p)).transpose()?;
        let subnet_membership_json = serde_json::to_string(&node.base.subnet_membership)?;
        let node_type_str = serde_json::to_string(&node.base.node_type)?;
        let last_seen_str = node.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_json = serde_json::to_string(&node.base.target)?;
        let open_ports_json = serde_json::to_string(&node.base.open_ports)?;
        let detected_services_json = serde_json::to_string(&node.base.detected_services)?;
        let current_status_json = serde_json::to_string(&node.base.current_status)?;

        sqlx::query(
            r#"
            INSERT INTO nodes (
                id, name, target, description,
                node_type, capabilities, assigned_tests, monitoring_interval,
                node_groups, position, current_status, subnet_membership,
                open_ports, detected_services, mac_address,
                last_seen, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&node.id)
        .bind(&node.base.name)
        .bind(target_json)
        .bind(&node.base.description)
        .bind(node_type_str)
        .bind(capabilities_json)
        .bind(assigned_tests_json)
        .bind(node.base.monitoring_interval)
        .bind(node_groups_json)
        .bind(position_json)
        .bind(current_status_json)
        .bind(subnet_membership_json)
        .bind(open_ports_json)
        .bind(detected_services_json)
        .bind(&node.base.mac_address)
        .bind(last_seen_str)
        .bind(&node.created_at)
        .bind(&node.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Node>> {
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
        let capabilities_json = serde_json::to_string(&node.base.capabilities)?;
        let assigned_tests_json = serde_json::to_string(&node.base.assigned_tests)?;
        let node_groups_json = serde_json::to_string(&node.base.node_groups)?;
        let position_json = node.base.position.as_ref().map(|p| serde_json::to_string(p)).transpose()?;
        let subnet_membership_json = serde_json::to_string(&node.base.subnet_membership)?;
        let node_type_str = serde_json::to_string(&node.base.node_type)?;
        let last_seen_str = node.last_seen.as_ref().map(|dt| dt.to_rfc3339());
        let target_json = serde_json::to_string(&node.base.target)?;
        let open_ports_json = serde_json::to_string(&node.base.open_ports)?;
        let detected_services_json = serde_json::to_string(&node.base.detected_services)?;
        let current_status_json = serde_json::to_string(&node.base.current_status)?;

        sqlx::query(
            r#"
            UPDATE nodes SET 
                name = ?, target = ?, description = ?,
                node_type = ?, capabilities = ?, assigned_tests = ?, monitoring_interval = ?,
                node_groups = ?, position = ?, current_status = ?, subnet_membership = ?,
                open_ports = ?, detected_services = ?, mac_address = ?,
                last_seen = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&node.base.name)
        .bind(target_json)
        .bind(&node.base.description)
        .bind(node_type_str)
        .bind(capabilities_json)
        .bind(assigned_tests_json)
        .bind(node.base.monitoring_interval)
        .bind(node_groups_json)
        .bind(position_json)
        .bind(current_status_json)
        .bind(subnet_membership_json)
        .bind(open_ports_json)
        .bind(detected_services_json)
        .bind(last_seen_str)
        .bind(&node.updated_at)
        .bind(&node.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM nodes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_group(&self, group_id: &str) -> Result<Vec<Node>> {
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
    let capabilities_json: String = row.get("capabilities");
    let assigned_tests_json: String = row.get("assigned_tests");
    let node_groups_json: String = row.get("node_groups");
    let subnet_membership_json: String = row.get("subnet_membership");
    let current_status_json: String = row.get("current_status");
    let target_json: String = row.get("target");
    let open_ports_json: String = row.get("open_ports");
    let detected_services_json: String = row.get("detected_services");
    
    let capabilities: Vec<NodeCapability> = serde_json::from_str(&capabilities_json)?;
    let assigned_tests: Vec<AssignedTest> = serde_json::from_str(&assigned_tests_json)?;
    let node_groups: Vec<String> = serde_json::from_str(&node_groups_json)?;
    let subnet_membership: Vec<String> = serde_json::from_str(&subnet_membership_json)?;
    let current_status: NodeStatus = serde_json::from_str(&current_status_json)?;
    let target: NodeTarget = serde_json::from_str(&target_json)?;
    let node_type: NodeType = serde_json::from_str(row.get("node_type"))?;
    let open_ports: Vec<u16> = serde_json::from_str(&open_ports_json)?;
    let detected_services: Vec<DetectedService> = serde_json::from_str(&detected_services_json)?;
    
    let position: Option<GraphPosition> = match row.get::<Option<String>, _>("position") {
        Some(pos_str) => Some(serde_json::from_str(&pos_str)?),
        None => None,
    };
    
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
            description: row.get("description"),
            node_type,
            capabilities,
            detected_services,
            open_ports,
            assigned_tests,
            mac_address: row.get("mac_address"),
            monitoring_interval: row.get("monitoring_interval"),
            node_groups,
            position,
            current_status,
            subnet_membership,
        }        
    })
}