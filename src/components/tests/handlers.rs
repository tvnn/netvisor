use axum::{
    extract::{State},
    response::Json,
    routing::{get},
    Router,
};
use strum::IntoEnumIterator;
use std::sync::Arc;
use crate::{
    api::{ApiResult, ApiResponse},
    components::{
        tests::types::{Test},
    },
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/types", get(get_test_types))
}

#[derive(serde::Serialize)]
pub struct TestTypeInfo {
    pub test_type: String,
    pub display_name: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub node_types: Vec<String>,
    pub node_target_types: Vec<String>
}

/// Get all available test types with their metadata
async fn get_test_types(
    State(_state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<TestTypeInfo>>>> {
    
    let test_info: Vec<TestTypeInfo> = Test::iter().map(|test| {
        let requirements = test.get_requirements();
        
        TestTypeInfo {
            display_name: test.display_name().to_string(),
            description: test.description().to_string(),
            capabilities: requirements.node_capabilities
                .unwrap_or_default()
                .iter()
                .map(|c| format!("{:?}", c))
                .collect(),
            node_types: requirements.node_types
                .unwrap_or_default()
                .iter()
                .map(|t| t.display_name().to_string())
                .collect(),
            node_target_types: requirements.node_target_types
                .unwrap_or_default()
                .iter()
                .map(|t| t.variant_name())
                .collect(),
            test_type: test.variant_name(),
        }
    }).collect();
    
    Ok(Json(ApiResponse::success(test_info)))
}