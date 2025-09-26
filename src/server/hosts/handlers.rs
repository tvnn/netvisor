use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use futures::future::try_join_all;
use itertools::{Itertools, Either};
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
        config::AppState, hosts::types::{api::{HostWithServicesRequest}, base::Host}, services::types::base::Service, shared::types::api::{ApiError, ApiResponse, ApiResult}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_host))
        .route("/", get(get_all_hosts))
        .route("/", put(update_host))
        .route("/:id", delete(delete_host))
        .route("/:destination_host/consolidate/:other_host", put(consolidate_hosts))

}

async fn create_host(
    State(state): State<Arc<AppState>>,
    Json(request): Json<HostWithServicesRequest>,
) -> ApiResult<Json<ApiResponse<Host>>> {
    let host_service = &state.services.host_service;
    let service_service = &state.services.service_service;

    let request_host = request.host.clone();

    // Create host first (handles duplicates via upsert_host)
    let mut created_host = host_service.create_host(request.host.base).await?;

    // Create services, handling case where created_service was upserted from host in request instead of created anew
    let service_futures = request.services.into_iter().map(|mut service| {
        service.base.interface_bindings = service.base.interface_bindings.iter().filter_map(|b| {
            if let Some(original_binding) = request_host.get_interface(b) {
                return created_host.base.interfaces.iter().find_map(|i| if i == original_binding {Some(i.id)} else {None});
            }
            None
        })
        .collect();
        service.base.host_id = created_host.id;
        service_service.create_service(service)
    });

    let services = try_join_all(service_futures).await?;

    // Add all successfully created/found services to the host
    for service in &services {
        if !created_host.base.services.contains(&service.id) {
            created_host.base.services.push(service.id);
        }
    }

    let host_with_final_services = host_service.update_host(created_host).await?;
    
    Ok(Json(ApiResponse::success(host_with_final_services)))
}

async fn get_all_hosts(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Host>>>> {
    let service = &state.services.host_service;
    let hosts = service.get_all_hosts().await?;
    
    Ok(Json(ApiResponse::success(hosts)))
}

async fn update_host(
    State(state): State<Arc<AppState>>,
    Json(mut request): Json<HostWithServicesRequest>,
) -> ApiResult<Json<ApiResponse<Host>>> {

    let host_service = &state.services.host_service;
    let service_service = &state.services.service_service;

    let (create_futures, update_futures): (Vec<_>, Vec<_>) = request.services.into_iter().partition_map(|s| {
        if s.id == Uuid::nil() {
            let service = Service::new(s.base);
            Either::Left(service_service.create_service(service))
        } else {
            Either::Right(service_service.update_service(s))
        }
    });

    let created_services= try_join_all(create_futures).await?;
    let updated_services= try_join_all(update_futures).await?;
    
    request.host.base.services = created_services.iter().chain(updated_services.iter()).map(|s| s.id).collect();

    let updated_host = host_service.update_host(
        request.host, 
        ).await?;
    
    Ok(Json(ApiResponse::success(updated_host)))
}

async fn consolidate_hosts(
    State(state): State<Arc<AppState>>,
    Path((destination_host_id, other_host_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<ApiResponse<Host>>> {

    let host_service = &state.services.host_service;
    
    let destination_host = host_service.get_host(&destination_host_id).await?.ok_or_else(|| ApiError::not_found("Could not find host"))?;
    let other_host = host_service.get_host(&other_host_id).await?.ok_or_else(|| ApiError::not_found("Could not find host to convert"))?;

    let updated_host = host_service.consolidate_hosts(destination_host, other_host).await?;
    
    Ok(Json(ApiResponse::success(updated_host)))
}

async fn delete_host(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.host_service;
    
    // Check if host exists
    if service.get_host(&id).await?.is_none() {
        return Err(ApiError::not_found(&format!("Host '{}' not found", &id)));
    }
    
    service.delete_host(&id, false).await?;
    
    Ok(Json(ApiResponse::success(())))
}