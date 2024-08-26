use axum::http::StatusCode;
use axum::response::sse::Event;
use axum_macros::FromRef;
use dto::notification::{CompleteNotification, NotificationData};
use futures::channel::mpsc::unbounded;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::SinkExt;
use neo4rs::Graph;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;

use entity::{prelude::*, *};
use sea_orm::prelude::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};

type NotificationChannels = Arc<RwLock<HashMap<i64, UnboundedSender<Result<Event, Infallible>>>>>;

#[derive(Clone, FromRef)]
pub struct NotificationService {
    neo4j: Arc<Graph>,
    channels: NotificationChannels,
    postgres: sea_orm::DatabaseConnection,
}

impl NotificationService {
    pub async fn dismiss_notification(
        &self,
        user_id: i64,
        notification_id: i64,
    ) -> Result<(), StatusCode> {
        let _ = Notification::find()
            .filter(notification::Column::Id.eq(notification_id))
            .filter(notification::Column::AccountId.eq(user_id))
            .one(&self.postgres)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
            .delete(&self.postgres)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
    }

    pub async fn get_remaining_notifications(
        &self,
        user_id: i64,
    ) -> Result<Vec<CompleteNotification>, StatusCode> {
        let notifications = Notification::find()
            .filter(notification::Column::AccountId.eq(user_id))
            .order_by_desc(notification::Column::Date)
            .all(&self.postgres)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut results = vec![];
        for value in notifications {
            let not = CompleteNotification {
                id: value.id,
                date: value.date,
                notification_data: value.content,
            };
            results.push(not);
        }

        Ok(results)
    }

    pub async fn send_notification<T: Serialize>(
        &self,
        user_id: i64,
        notification: NotificationData<T>,
    ) -> Result<i64, StatusCode> {
        let json = serde_json::to_string(&notification).unwrap();

        let model = notification::ActiveModel {
            date: Set(chrono::Utc::now().naive_local()),
            content: Set(Json::from(json)),
            account_id: Set(user_id),
            ..Default::default()
        };

        let ret = Notification::insert(model)
            .exec_with_returning(&self.postgres)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let complete = CompleteNotification {
            id: ret.id,
            date: ret.date,
            notification_data: ret.content,
        };

        let c = self.channels.read().await;
        if let Some(mut tx) = c.get(&user_id) {
            if let Ok(event) = Event::default().json_data(complete) {
                let _ = tx.send(Ok(event)).await;
            }
        }

        Ok(ret.id)
    }
}

impl NotificationService {
    pub async fn subscribe_to_notifications(
        &mut self,
        user_id: i64,
    ) -> UnboundedReceiver<Result<Event, Infallible>> {
        let (tx, rx) = unbounded();
        self.channels.write().await.insert(user_id, tx);

        rx
    }
}

impl NotificationService {
    pub fn new(neo4j: Arc<Graph>, postgres: sea_orm::DatabaseConnection) -> Self {
        Self {
            neo4j,
            channels: Arc::new(RwLock::new(HashMap::new())),
            postgres,
        }
    }
}
