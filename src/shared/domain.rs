use async_trait::async_trait;
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// Core entity trait - everything that can be stored
pub trait Entity: Send + Sync + Clone + Debug + Serialize + DeserializeOwned + 'static {
    type Id: Send + Sync + Clone + Debug + Serialize + DeserializeOwned + 'static;
    
    fn id(&self) -> &Self::Id;
    fn set_id(&mut self, id: Self::Id);
}

/// Request types that can create entities
pub trait CreateRequest<E: Entity>: Send + Sync + DeserializeOwned + 'static {
    fn into_entity(self) -> E;
}

/// Request types that can update entities  
pub trait UpdateRequest<E: Entity>: Send + Sync + DeserializeOwned + 'static {
    fn apply_to(self, entity: &mut E);
}

/// Response types for API endpoints
pub trait ApiResponse<E: Entity>: Send + Sync + Serialize + From<E> + 'static {}

// Blanket implementation for any type that meets the requirements
impl<T, E> ApiResponse<E> for T 
where 
    T: Send + Sync + Serialize + From<E> + 'static,
    E: Entity,
{}
