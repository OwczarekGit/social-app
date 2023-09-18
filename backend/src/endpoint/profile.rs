use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::routing::put;
use serde::{Deserialize, Serialize};
use crate::{AppState};
use crate::app_state::ActiveUser;
use crate::service::profile::ProfileService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/username", put(change_username).get(get_my_profile))
}

pub async fn change_username(
    Extension(user): Extension<ActiveUser>,
    State(profile_service): State<ProfileService>,
    Json(request): Json<ChangeUsernameRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    profile_service.change_username(user.id, &request.username).await
}

pub async fn get_my_profile(
    Extension(user): Extension<ActiveUser>,
    State(profile_service): State<ProfileService>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(profile_service.get_profile(user.id).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeUsernameRequest {
    pub username: String,
}