use crate::app_state::AppState;
use crate::service::domain::DomainService;
use crate::SysRes;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::put;
use axum::{Json, Router};
use dto::domain::*;

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/system", put(set_system_domain).get(get_system_domain))
        .route("/image", put(set_image_domain).get(get_image_domain))
}

pub async fn get_image_domain(
    State(domain_service): State<DomainService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        domain_service
            .get_image_domain()
            .await?
            .map(|v| GetVariableResponse { value: v }),
    ))
}

pub async fn get_system_domain(
    State(domain_service): State<DomainService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        domain_service
            .get_system_domain()
            .await?
            .map(|v| GetVariableResponse { value: v }),
    ))
}

pub async fn set_system_domain(
    State(domain_service): State<DomainService>,
    Json(request): Json<SetVariableRequest>,
) -> SysRes<()> {
    domain_service.set_system_domain(&request.value).await
}

pub async fn set_image_domain(
    State(mut domain_service): State<DomainService>,
    Json(request): Json<SetVariableRequest>,
) -> SysRes<()> {
    domain_service.set_image_domain(&request.value).await
}
