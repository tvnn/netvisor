use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// API Requests and Responses
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeGroupRequest {
    #[serde(flatten)]
    pub group: NodeGroupBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeGroupRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub node_sequence: Option<Vec<String>>,  // Ordered diagnostic sequence
    pub auto_diagnostic_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeGroupResponse {
    pub group: NodeGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeGroupListResponse {
    pub groups: Vec<NodeGroup>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGroupBase {
    pub name: String,
    pub description: Option<String>,
    pub node_sequence: Vec<String>,  // Ordered diagnostic sequence
    pub auto_diagnostic_enabled: bool, // Default: true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGroup {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: NodeGroupBase,
}

impl NodeGroup {
    pub fn new(base: NodeGroupBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            base,
        }
    }

    pub fn from_name(name: String) -> Self {
        let base = NodeGroupBase {
            name,
            description: None,
            node_sequence: Vec::new(),
            auto_diagnostic_enabled: true,
        };

        Self::new(base)
    }

    // Setters with timestamp updates
    pub fn set_auto_diagnostic_enabled(&mut self, enabled: bool) {
        self.base.auto_diagnostic_enabled = enabled;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn set_description(&mut self, description: String) {
        self.base.description = Some(description);
        self.updated_at = chrono::Utc::now();
    }

    // Node sequence management
    pub fn add_node(&mut self, node_id: String) {
        if !self.base.node_sequence.contains(&node_id) {
            self.base.node_sequence.push(node_id);
            self.updated_at = chrono::Utc::now();
        }
    }
    
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        let initial_len = self.base.node_sequence.len();
        self.base.node_sequence.retain(|id| id != node_id);
        if self.base.node_sequence.len() != initial_len {
            self.updated_at = chrono::Utc::now();
            true
        } else {
            false
        }
    }
    
    pub fn reorder_nodes(&mut self, new_sequence: Vec<String>) {
        self.base.node_sequence = new_sequence;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn move_node_up(&mut self, node_id: &str) -> bool {
        if let Some(index) = self.base.node_sequence.iter().position(|id| id == node_id) {
            if index > 0 {
                self.base.node_sequence.swap(index - 1, index);
                self.updated_at = chrono::Utc::now();
                return true;
            }
        }
        false
    }
    
    pub fn move_node_down(&mut self, node_id: &str) -> bool {
        if let Some(index) = self.base.node_sequence.iter().position(|id| id == node_id) {
            if index < self.base.node_sequence.len() - 1 {
                self.base.node_sequence.swap(index, index + 1);
                self.updated_at = chrono::Utc::now();
                return true;
            }
        }
        false
    }

    // Read-only methods (no setters needed)
    pub fn contains_node(&self, node_id: &str) -> bool {
        self.base.node_sequence.contains(&node_id.to_string())
    }

    pub fn node_count(&self) -> usize {
        self.base.node_sequence.len()
    }
}