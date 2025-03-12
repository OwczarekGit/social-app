use crate::active_user::ActiveUser;
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use dto::chat::SendMessageToFriendRequest;
use dto::notification::{NotificationData, NotificationType};

use crate::service::chat::ChatService;
use crate::service::notification::NotificationService;
use crate::SysRes;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/friend", post(send_message_to_friend))
        .route("/friend/{friend_id}", get(get_friend_conversation_messages))
}

pub async fn send_message_to_friend(
    user: ActiveUser,
    State(chat_service): State<ChatService>,
    State(notification_service): State<NotificationService>,
    Json(request): Json<SendMessageToFriendRequest>,
) -> SysRes<impl IntoResponse> {
    let result = chat_service
        .send_message_to_friend(user.id, request.friend_id, &request.message)
        .await?;

    let notification = NotificationData {
        notification_type: NotificationType::MESSAGE,
        data: result.clone(),
    };

    let _ = notification_service
        .send_notification(request.friend_id, notification)
        .await;

    Ok(Json(result))
}

pub async fn get_friend_conversation_messages(
    user: ActiveUser,
    State(chat_service): State<ChatService>,
    Path(friend_id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        chat_service
            .get_friend_conversation_messages(user.id, friend_id)
            .await?,
    ))
}
