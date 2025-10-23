use crate::server::discovery::types::base::EntitySource;
use crate::server::shared::constants::Entity;
use crate::server::shared::types::api::deserialize_empty_string_as_none;
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter, IntoStaticStr};
use uuid::Uuid;
use validator::Validate;

#[derive(
    Copy,
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Hash,
    Default,
    PartialEq,
    Eq,
    EnumIter,
    IntoStaticStr,
    EnumDiscriminants,
)]
#[strum_discriminants(derive(IntoStaticStr))]
pub enum GroupType {
    #[default]
    NetworkPath,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct GroupBase {
    #[validate(length(min = 0, max = 100))]
    pub name: String,
    pub network_id: Uuid,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    pub service_bindings: Vec<Uuid>,
    pub group_type: GroupType,
    pub source: EntitySource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: GroupBase,
}

impl Group {
    pub fn new(base: GroupBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            base,
        }
    }
}

impl HasId for GroupType {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for GroupType {
    fn color(&self) -> &'static str {
        match self {
            GroupType::NetworkPath => Entity::Group.color(),
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            GroupType::NetworkPath => "Route",
        }
    }
}

impl TypeMetadataProvider for GroupType {
    fn name(&self) -> &'static str {
        match self {
            GroupType::NetworkPath => "Network Path",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            GroupType::NetworkPath => {
                "Path of network traffic between sources. Edge will be directed based on service order."
            }
        }
    }
}
