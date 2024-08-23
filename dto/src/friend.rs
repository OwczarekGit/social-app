use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFriendRequest {
    pub phrase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendNotificationData {
    pub user_id: i64,
}
