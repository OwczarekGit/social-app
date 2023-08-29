use std::convert::Infallible;
use std::time::Duration;
use axum::extract::State;
use axum::response::{IntoResponse, Sse};
use axum::response::sse::{Event, KeepAlive};
use axum::{Extension, Router};
use axum::routing::get;
use futures::Stream;
use serde::{Serialize};
use crate::{ActiveUserId, AppState};
use crate::service::notification::NotificationService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/subscribe", get(subscribe_to_notifications))
        .route("/test", get(send_notification))
}


pub async fn subscribe_to_notifications(
    Extension(user): Extension<ActiveUserId>,
    State(mut notification_service): State<NotificationService>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = notification_service.subscribe_to_notifications(user.0).await;

    Sse::new(rx)
        .keep_alive(
            KeepAlive::new()
                .interval(Duration::from_secs(60))
                .text("keep-alive")
        )
}

pub async fn send_notification(
    Extension(user): Extension<ActiveUserId>,
    State(notification_service): State<NotificationService>,
) -> impl IntoResponse {
    let n = Notification {
        notification_type: NotificationType::MESSAGE,
        data: user.0
    };

    let _ = notification_service.send_notification(user.0, n).await;
}


#[derive(Serialize, Copy, Clone, Debug)]
pub struct Notification<T: Serialize> {
    pub notification_type: NotificationType,
    pub data: T
}

#[derive(Serialize, Clone, Debug)]
pub struct FriendNotificationData {
    pub user_id: i64,
    pub message: String,
}

#[derive(Serialize, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum NotificationType {
    MESSAGE,
    FRIEND_REQUEST,
}