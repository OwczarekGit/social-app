use axum::{response::IntoResponse, Json, Router, routing::post, extract::{State, Query}, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::{AppState, service::{account::AccountService, email::EmailService}};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(register_account))
        .route("/activate", post(activate_account))
}

pub async fn register_account(
    State(mut account_service): State<AccountService>,
    State(email_service): State<EmailService>,
    Json(request): Json<RegistrationRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    //TODO: Verify that email is a valid email address.
    let (email, key) = account_service.register_account(&request.email, &request.password).await?;
    email_service.send_activation_mail(&email, &key);
    Ok(())
}

pub async fn activate_account(
    State(mut account_service): State<AccountService>,
    Query(params): Query<AccountActivationParams>,
) -> Result<impl IntoResponse, StatusCode> {
    account_service.activate_account(&params.email, &params.key).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountActivationParams {
    email: String,
    key: String,
}