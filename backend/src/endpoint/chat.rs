use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use crate::{ActiveUser, AppState};

use crate::Result;
use crate::service::chat::ChatService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(send_message_to_friend))
}

pub async fn send_message_to_friend(
    Extension(user): Extension<ActiveUser>,
    State(chat_service): State<ChatService>,
    Json(request): Json<SendMessageToFriendRequest>,
) -> Result<impl IntoResponse> {
    chat_service.send_message_to_friend(user.id, request.friend_id, &request.message).await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageToFriendRequest {
    pub message: String,
    pub friend_id: i64,
}