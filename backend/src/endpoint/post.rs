use axum::{Router, response::IntoResponse, http::StatusCode, Json, routing::post, Extension};
use axum::extract::State;
use serde::{Serialize, Deserialize};

use crate::{AppState, ActiveUserId};
use crate::service::post::PostService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_post))
} 

pub async fn create_post(
    State(post_service): State<PostService>,
    Extension(ActiveUserId(id)): Extension<ActiveUserId>,
    Json(request): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    post_service.create_post(id, &request.content).await?;
    dbg!(id);
    Ok(Json(request))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostRequest {
    content: String,
}