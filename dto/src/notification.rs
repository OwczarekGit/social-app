use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Copy, Clone, Debug)]
pub struct NotificationData<T: Serialize> {
    pub notification_type: NotificationType,
    pub data: T,
}

#[derive(Serialize, Clone, Debug)]
pub struct CompleteNotification {
    pub id: i64,
    pub date: NaiveDateTime,
    pub notification_data: Value,
}

#[derive(Serialize, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum NotificationType {
    MESSAGE,
    FRIEND_REQUEST,
}
