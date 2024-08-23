use axum::routing::{delete, put};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use dto::account::*;
use tower_cookies::{Cookie, Cookies};

use crate::SysRes;

use crate::app_state::ActiveUser;
use crate::{
    service::{account::AccountService, email::EmailService},
    AppState,
};

pub static SESSION_COOKIE_NAME: &str = "JSESSIONID";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(register_account))
        .route("/activate", post(activate_account))
        .route("/login", post(login))
        .route("/logout", delete(logout))
}

pub async fn register_account(
    State(mut account_service): State<AccountService>,
    State(email_service): State<EmailService>,
    Json(request): Json<RegistrationRequest>,
) -> SysRes<impl IntoResponse> {
    //TODO: Verify that email is a valid email address.
    let (email, key) = account_service
        .register_account(&request.username, &request.email, &request.password)
        .await?;
    email_service.send_activation_mail(&email, &key);
    Ok(())
}

pub async fn activate_account(
    State(mut account_service): State<AccountService>,
    Query(params): Query<AccountActivationParams>,
) -> SysRes<impl IntoResponse> {
    account_service
        .activate_account(&params.email, &params.key)
        .await?;
    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(mut account_service): State<AccountService>,
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
) -> SysRes<impl IntoResponse> {
    let (key, role) = account_service
        .login(&request.email, &request.password)
        .await?;

    cookies.add(make_cookie(SESSION_COOKIE_NAME.to_string(), key, true));
    cookies.add(make_cookie("AUTH".to_owned(), "".to_string(), false));
    cookies.add(make_cookie("ROLE".to_owned(), role.to_string(), false));

    Ok(StatusCode::OK)
}

pub async fn logout(
    State(mut account_service): State<AccountService>,
    cookies: Cookies,
) -> impl IntoResponse {
    if let Some(key) = cookies.get(SESSION_COOKIE_NAME) {
        account_service.logout(key.value()).await;
    }

    cookies.remove(remove_cookie(SESSION_COOKIE_NAME.to_string()));
    cookies.remove(remove_cookie("AUTH".to_string()));
    cookies.remove(remove_cookie("ROLE".to_string()));

    StatusCode::OK
}

pub fn logged_in_routes() -> Router<AppState> {
    Router::new().route("/password", put(change_password))
}

pub async fn change_password(
    user: ActiveUser,
    State(account_service): State<AccountService>,
    Json(request): Json<ChangePasswordRequest>,
) -> SysRes<impl IntoResponse> {
    account_service
        .change_password(user.id, &request.old_password, &request.new_password)
        .await
}

pub fn make_cookie(name: String, value: String, http: bool) -> Cookie<'static> {
    Cookie::build((name, value))
        .http_only(http)
        .path("/")
        .build()
}

pub fn remove_cookie(name: String) -> Cookie<'static> {
    Cookie::build(name).path("/").build()
}
