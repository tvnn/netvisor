use axum::{routing::post, Json, Router};
use std::{sync::Arc, collections::HashMap};
use strum::IntoEnumIterator;
use crate::server::{
        capabilities::types::{api::CapabilityFormRequest, base::{Capability, CapabilityDiscriminants}, forms::CapabilityConfigForm}, config::AppState, shared::types::api::{ApiResponse, ApiResult}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/forms", post(get_capability_form))  // Single endpoint handles all cases
}

pub async fn get_capability_form(
    Json(request): Json<CapabilityFormRequest>,
) -> ApiResult<Json<ApiResponse<HashMap<CapabilityDiscriminants, CapabilityConfigForm>>>> {
        
    let mut forms = HashMap::new();
    
    // Determine which test types to generate schemas for
    let capability_types = match request.capability_types {
        Some(types) => types,  // Specific types requested
        None => CapabilityDiscriminants::iter().collect(),  // All types
    };
    
    for discriminant in capability_types {
        let ports = discriminant.discovery_ports();
        let port = if ports.len() > 0 {Some(ports[0])} else {None};
        if let Some(capability) = Capability::from_port(port) {
            let schema = capability.generate_form();
            forms.insert(discriminant, schema);
        }
    }
    
    Ok(Json(ApiResponse::success( forms)))
}