use axum::{extract::State, routing::post, Json, Router};
use std::{sync::Arc, collections::HashMap};
use strum::IntoEnumIterator;
use crate::server::{
        capabilities::types::{api::CapabilityFormRequest, base::{Capability, CapabilityDiscriminants}, forms::CapabilityConfigForm}, config::AppState, nodes::service::NodeService, shared::types::api::{ApiError, ApiResponse, ApiResult}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/forms", post(get_capability_form))  // Single endpoint handles all cases
}

pub async fn get_capability_form(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CapabilityFormRequest>,
) -> ApiResult<Json<ApiResponse<HashMap<CapabilityDiscriminants, CapabilityConfigForm>>>> {
    
    let node_service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    let available_nodes = node_service.get_all_nodes().await
        .map_err(|e| ApiError::internal_error(&format!("Failed to load nodes: {}", e)))?;
    
    let mut forms = HashMap::new();
    
    // Determine which test types to generate schemas for
    let capability_types = match request.capability_types {
        Some(types) => types,  // Specific types requested
        None => CapabilityDiscriminants::iter().collect(),  // All types
    };
    
    for discriminant in capability_types {
        let capability = Capability::default_for_discriminant(discriminant);
        let schema = capability.generate_form(&request.node_context, &available_nodes);
        forms.insert(discriminant, schema);
    }
    
    Ok(Json(ApiResponse::success( forms)))
}