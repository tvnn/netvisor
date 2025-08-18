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
        tests::{
            types::{TestType}
        },
    },
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/types", get(get_test_types))
}

#[derive(serde::Serialize)]
pub struct TestTypeInfo {
    pub test_type: TestType,
    pub display_name: String,
    pub description: String,
    pub helpful_capabilities: Vec<String>,
    pub ideal_node_types: Vec<String>,
}

/// Get all available test types with their metadata
async fn get_test_types(
    State(_state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<TestTypeInfo>>>> {
    
    let test_info: Vec<TestTypeInfo> = TestType::iter().map(|test_type| {
        let recommendations = test_type.get_recommendations();
        
        TestTypeInfo {
            display_name: test_type.display_name().to_string(),
            description: test_type.description().to_string(),
            helpful_capabilities: recommendations.helpful_capabilities.iter().map(|c| format!("{:?}", c)).collect(),
            ideal_node_types: recommendations.ideal_node_types.iter().map(|t| t.display_name().to_string()).collect(),
            test_type,
        }
    }).collect();
    
    Ok(Json(ApiResponse::success(test_info)))
}