use axum::{response::IntoResponse, Json, Router, routing::post, extract::{State, Query}, http::StatusCode};
use serde::{Serialize, Deserialize};
use tower_cookies::{Cookies, Cookie};

use crate::{AppState, service::{account::AccountService, email::EmailService}};

static SESSION_COOKIE_NAME: &str = "JSESSIONID";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(register_account))
        .route("/activate", post(activate_account))
        .route("/login", post(login))
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
    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(mut account_service): State<AccountService>,
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let key = account_service.login(&request.email, &request.password).await?;

    let cookie = Cookie::build(SESSION_COOKIE_NAME, key)
        .http_only(true)
        .path("/")
        .finish();

    cookies.add(cookie);

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
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