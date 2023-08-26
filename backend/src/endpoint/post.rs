use axum::{Router, response::IntoResponse, http::StatusCode, Json, routing::post, Extension};
use serde::{Serialize, Deserialize};

use crate::{AppState, ActiveUserId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_post))
} 

pub async fn create_post(
    Extension(ActiveUserId(id)): Extension<ActiveUserId>,
    Json(request): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    dbg!(id);
    Ok(Json(request))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostRequest {
    content: String,
}