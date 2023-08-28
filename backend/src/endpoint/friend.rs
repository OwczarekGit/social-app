use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::{ActiveUserId, AppState};
use crate::service::friend::FriendService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/request/pending", get(get_pending_friend_requests))
        .route("/request/accept/:id", post(accept_friend_request))
}

pub async fn get_pending_friend_requests(
    Extension(user): Extension<ActiveUserId>,
    State(friend_service): State<FriendService>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(friend_service.get_pending_friend_requests(user.0).await?))
}

pub async fn accept_friend_request(
    Extension(user): Extension<ActiveUserId>,
    State(friend_service): State<FriendService>,
    Path(requester_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    friend_service.accept_friend_request(user.0, requester_id).await
}