use crate::service::activation::ActivationService;
use crate::AppState;
use crate::SysRes;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use dto::activation::SetActivationEmailTemplateRequest;

pub fn admin_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(get_current_activation_email_template).put(set_current_activation_email_template),
    )
}

pub async fn get_current_activation_email_template(
    State(activation_service): State<ActivationService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        activation_service
            .get_current_activation_mail_template()
            .await?,
    ))
}

pub async fn set_current_activation_email_template(
    State(activation_service): State<ActivationService>,
    Json(request): Json<SetActivationEmailTemplateRequest>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        activation_service
            .set_current_activation_mail_template(&request.content)
            .await?,
    ))
}
