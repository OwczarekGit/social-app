use std::convert::Infallible;
use std::time::Duration;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Sse};
use axum::response::sse::{Event, KeepAlive};
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::routing::{delete, get};
use chrono::NaiveDateTime;
use futures::Stream;
use serde::{Serialize};
use serde_json::Value;
use crate::{ActiveUserId, AppState};
use crate::service::notification::NotificationService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/subscribe", get(subscribe_to_notifications))
        .route("/", get(get_remaining_notifications))
        .route("/:id", delete(dismiss_notification))
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

pub async fn get_remaining_notifications(
    Extension(user): Extension<ActiveUserId>,
    State(notification_service): State<NotificationService>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(notification_service.get_remaining_notifications(user.0).await?))
}

pub async fn dismiss_notification(
    Extension(user): Extension<ActiveUserId>,
    State(notification_service): State<NotificationService>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    notification_service.dismiss_notification(user.0, id).await
}


#[derive(Serialize, Copy, Clone, Debug)]
pub struct NotificationData<T: Serialize> {
    pub notification_type: NotificationType,
    pub data: T
}

#[derive(Serialize, Clone, Debug)]
pub struct CompleteNotification {
    pub id: i64,
    pub date: NaiveDateTime,
    pub notification_data: Value
}


#[derive(Serialize, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum NotificationType {
    MESSAGE,
    FRIEND_REQUEST,
}