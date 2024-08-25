use crate::image_domain::ImageDomain;
use crate::service::friend::{FriendService, Profile};
use crate::service::notification::NotificationService;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use dto::friend::*;
use dto::notification::{NotificationData, NotificationType};

use crate::active_user::ActiveUser;
use crate::{Error, SysRes};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/request/pending", get(get_pending_friend_requests))
        .route("/request/accept/:requester_id", post(accept_friend_request))
        .route("/invite/:target_id", post(send_friend_request))
        .route("/list", get(get_friend_list))
        .route("/", get(search_users))
        .route("/:id", delete(remove_friend))
}

pub async fn search_users(
    user: ActiveUser,
    State(friend_service): State<FriendService>,
    Query(search_request): Query<SearchFriendRequest>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        friend_service
            .search_for_non_friends(user.id, &search_request.phrase)
            .await?,
    ))
}

pub async fn get_pending_friend_requests(
    user: ActiveUser,
    State(friend_service): State<FriendService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        friend_service.get_pending_friend_requests(user.id).await?,
    ))
}

pub async fn accept_friend_request(
    user: ActiveUser,
    State(friend_service): State<FriendService>,
    Path(requester_id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    friend_service
        .accept_friend_request(user.id, requester_id)
        .await
}

pub async fn send_friend_request(
    user: ActiveUser,
    State(friend_service): State<FriendService>,
    State(notification_service): State<NotificationService>,
    Path(target_id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    if user.id == target_id {
        return Err(Error::RelationAttemptToAddSelfAsFriend(user.id));
    }

    friend_service
        .send_friend_request(user.id, target_id)
        .await?;

    let notification = NotificationData {
        notification_type: NotificationType::FRIEND_REQUEST,
        data: FriendNotificationData { user_id: user.id },
    };

    let _ = notification_service
        .send_notification(target_id, notification)
        .await;
    Ok(())
}

pub async fn get_friend_list(
    image_domain: ImageDomain,
    user: ActiveUser,
    State(friend_service): State<FriendService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        friend_service
            .get_friend_list(user.id)
            .await?
            .into_iter()
            .map(|p| Profile {
                picture_url: format!("{}{}", image_domain.0, p.picture_url),
                ..p
            })
            .collect::<Vec<_>>(),
    ))
}

pub async fn remove_friend(
    user: ActiveUser,
    State(friend_service): State<FriendService>,
    Path(other_id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    friend_service.remove_friend(user.id, other_id).await
}
