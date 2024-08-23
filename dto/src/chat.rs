use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageToFriendRequest {
    pub friend_id: i64,
    pub message: String,
}
