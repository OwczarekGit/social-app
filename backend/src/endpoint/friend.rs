use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use crate::{ActiveUserId, AppState};
use crate::endpoint::notification::{NotificationData, NotificationType};
use crate::service::friend::FriendService;
use crate::service::notification::NotificationService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/request/pending", get(get_pending_friend_requests))
        .route("/request/accept/:requester_id", post(accept_friend_request))
        .route("/invite/:target_id", post(send_friend_request))
        .route("/", get(search_users))
}

pub async fn search_users(
    Extension(user): Extension<ActiveUserId>,
    State(friend_service): State<FriendService>,
    Query(search_request): Query<SearchFriendRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(friend_service.search_for_non_friends(user.0, &search_request.phrase).await?))
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

pub async fn send_friend_request(
    Extension(user): Extension<ActiveUserId>,
    State(friend_service): State<FriendService>,
    State(notification_service): State<NotificationService>,
    Path(target_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    if user.0 == target_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    friend_service.send_friend_request(user.0, target_id).await?;

    let notification = NotificationData {
        notification_type: NotificationType::FRIEND_REQUEST,
        data: FriendNotificationData {
            user_id: user.0,
        }
    };

    let _ = notification_service.send_notification(target_id, notification).await;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchFriendRequest {
    pub phrase: String
}

#[derive(Serialize, Clone, Debug)]
pub struct FriendNotificationData {
    pub user_id: i64,
}
