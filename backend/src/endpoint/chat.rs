use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use crate::{AppState};
use crate::app_state::ActiveUser;
use crate::endpoint::notification::{NotificationData, NotificationType};

use crate::Result;
use crate::service::chat::ChatService;
use crate::service::notification::NotificationService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/friend", post(send_message_to_friend))
        .route("/friend/:friend_id", get(get_friend_conversation_messages))
}

pub async fn send_message_to_friend(
    Extension(user): Extension<ActiveUser>,
    State(chat_service): State<ChatService>,
    State(notification_service): State<NotificationService>,
    Json(request): Json<SendMessageToFriendRequest>,
) -> Result<impl IntoResponse> {
    let result = chat_service.send_message_to_friend(user.id, request.friend_id, &request.message).await?;

    let notification = NotificationData {
        notification_type: NotificationType::MESSAGE,
        data: result.clone()
    };

    let _ = notification_service.send_notification(request.friend_id, notification).await;

    Ok(Json(result))
}

pub async fn get_friend_conversation_messages(
    Extension(user): Extension<ActiveUser>,
    State(chat_service): State<ChatService>,
    Path(friend_id): Path<i64>,
) -> Result<impl IntoResponse> {
    Ok(Json(chat_service.get_friend_conversation_messages(user.id, friend_id).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageToFriendRequest {
    pub friend_id: i64,
    pub message: String,
}