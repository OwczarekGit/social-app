use axum::extract::State;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, put};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::{Result};
use crate::service::domain::DomainService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/system", get(get_system_domain))
        .route("/image", get(get_image_domain))
}

pub async fn get_image_domain(
    State(domain_service): State<DomainService>,
) -> Result<impl IntoResponse> {
    Ok(
        Json(
            domain_service.get_image_domain().
                await?
                .map(|v| GetVariableResponse { value: v })
        )
    )
}

pub async fn get_system_domain(
    State(domain_service): State<DomainService>,
) -> Result<impl IntoResponse> {
    Ok(
        Json(
            domain_service.get_system_domain().
                await?
                .map(|v| GetVariableResponse { value: v })
        )
    )
}


pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/system", put(set_system_domain))
        .route("/image", put(set_image_domain))
}

pub async fn set_system_domain(
    State(domain_service): State<DomainService>,
    Json(request): Json<SetVariableRequest>,
) -> Result<()> {
    domain_service.set_system_domain(&request.value).await
}

pub async fn set_image_domain(
    State(domain_service): State<DomainService>,
    Json(request): Json<SetVariableRequest>,
) -> Result<()> {
    domain_service.set_image_domain(&request.value).await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetVariableRequest {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVariableResponse {
    pub value: String,
}
