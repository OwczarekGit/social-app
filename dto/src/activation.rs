use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetActivationEmailTemplateRequest {
    pub content: String,
}
