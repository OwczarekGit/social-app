use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{get};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::service::activation::ActivationService;
use crate::{Result};


pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_current_activation_email_template).put(set_current_activation_email_template))
}

pub async fn get_current_activation_email_template(
    State(activation_service): State<ActivationService>,
) -> Result<impl IntoResponse> {
    Ok(Json(activation_service.get_current_activation_mail_template().await?))
}

pub async fn set_current_activation_email_template(
    State(activation_service): State<ActivationService>,
    Json(request): Json<SetActivationEmailTemplateRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(activation_service.set_current_activation_mail_template(&request.content).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetActivationEmailTemplateRequest {
    content: String,
}
