use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use axum::response::sse::Event;
use axum_macros::FromRef;
use neo4rs::Graph;
use futures::channel::mpsc::{UnboundedSender, UnboundedReceiver};
use futures::channel::mpsc::unbounded;
use futures::SinkExt;
use serde::Serialize;
use tokio::sync::RwLock;

#[derive(Clone, FromRef)]
pub struct NotificationService {
    neo4j: Arc<Graph>,
    channels: Arc<RwLock<HashMap<i64, UnboundedSender<Result<Event, Infallible>>>>>
}

impl NotificationService {
    pub async fn send_notification(&self, user_id: i64, notification: impl Serialize) -> bool {
        let c = self.channels.read().await;
        if let Some(mut tx) = c.get(&user_id) {
            if let Ok(event) = Event::default().json_data(notification) {
                let _ = tx.send(Ok(event)).await;
                return true
            }
        }

        false
    }
}

impl NotificationService {
    pub async fn subscribe_to_notifications(&mut self, user_id: i64) -> UnboundedReceiver<Result<Event, Infallible>> {
        let (tx, rx) = unbounded();
        self.channels.write().await.insert(user_id, tx);

        rx
    }
}

impl NotificationService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self {neo4j, channels: Arc::new(RwLock::new(HashMap::new()))}
    }
}