use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use crate::{ActiveUser, AppState};
use crate::endpoint::notification::{NotificationData, NotificationType};
use crate::service::friend::FriendService;
use crate::service::notification::NotificationService;

use crate::{Result, Error};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/request/pending", get(get_pending_friend_requests))
        .route("/request/accept/:requester_id", post(accept_friend_request))
        .route("/invite/:target_id", post(send_friend_request))
        .route("/list", get(get_friend_list))
        .route("/", get(search_users))
}

pub async fn search_users(
    Extension(user): Extension<ActiveUser>,
    State(friend_service): State<FriendService>,
    Query(search_request): Query<SearchFriendRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(friend_service.search_for_non_friends(user.id, &search_request.phrase).await?))
}

pub async fn get_pending_friend_requests(
    Extension(user): Extension<ActiveUser>,
    State(friend_service): State<FriendService>,
) -> Result<impl IntoResponse> {
    Ok(Json(friend_service.get_pending_friend_requests(user.id).await?))
}

pub async fn accept_friend_request(
    Extension(user): Extension<ActiveUser>,
    State(friend_service): State<FriendService>,
    Path(requester_id): Path<i64>,
) -> Result<impl IntoResponse> {
    friend_service.accept_friend_request(user.id, requester_id).await
}

pub async fn send_friend_request(
    Extension(user): Extension<ActiveUser>,
    State(friend_service): State<FriendService>,
    State(notification_service): State<NotificationService>,
    Path(target_id): Path<i64>,
) -> Result<impl IntoResponse> {
    if user.id == target_id {
        return Err(Error::RelationAttemptToAddSelfAsFriend(user.id));
    }

    friend_service.send_friend_request(user.id, target_id).await?;

    let notification = NotificationData {
        notification_type: NotificationType::FRIEND_REQUEST,
        data: FriendNotificationData {
            user_id: user.id,
        }
    };

    let _ = notification_service.send_notification(target_id, notification).await;
    Ok(())
}

pub async fn get_friend_list(
    Extension(user): Extension<ActiveUser>,
    State(friend_service): State<FriendService>,
) -> Result<impl IntoResponse> {
    Ok(Json(friend_service.get_friend_list(user.id).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchFriendRequest {
    pub phrase: String
}

#[derive(Serialize, Clone, Debug)]
pub struct FriendNotificationData {
    pub user_id: i64,
}
