use crate::active_user::ActiveUser;
use crate::service::notification::NotificationService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};
use axum::routing::{delete, get};
use axum::{Json, Router};
use futures::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tracing::log::debug;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/subscribe", get(subscribe_to_notifications))
        .route("/", get(get_remaining_notifications))
        .route("/{id}", delete(dismiss_notification))
}

pub async fn subscribe_to_notifications(
    user: ActiveUser,
    State(mut notification_service): State<NotificationService>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    debug!("Subscribed to notifications user {}.", user.id);
    let rx = notification_service
        .subscribe_to_notifications(user.id)
        .await;

    Sse::new(rx).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(60))
            .text("keep-alive"),
    )
}

pub async fn get_remaining_notifications(
    user: ActiveUser,
    State(notification_service): State<NotificationService>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(
        notification_service
            .get_remaining_notifications(user.id)
            .await?,
    ))
}

pub async fn dismiss_notification(
    user: ActiveUser,
    State(notification_service): State<NotificationService>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    notification_service.dismiss_notification(user.id, id).await
}
