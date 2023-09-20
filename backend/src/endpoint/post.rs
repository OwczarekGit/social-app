use axum::{Router, response::IntoResponse, Json, routing::post, Extension};
use axum::extract::{Path, State};
use axum::routing::get;
use serde::{Serialize, Deserialize};

use crate::{AppState};
use crate::app_state::ActiveUser;
use crate::service::post::PostService;

use crate::Result;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_posts))
        .route("/:id", get(get_user_posts))
        .route("/create", post(create_post))
}

pub async fn create_post(
    State(post_service): State<PostService>,
    Extension(user): Extension<ActiveUser>,
    Json(request): Json<CreatePostRequest>,
) -> Result<impl IntoResponse> {
    post_service.create_post(user.id, &request.content).await?;
    Ok(Json(request))
}

pub async fn get_user_posts(
    State(post_service): State<PostService>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    Ok(Json(post_service.get_posts_for_user(id).await?))
}

pub async fn get_my_posts(
    Extension(user): Extension<ActiveUser>,
    State(post_service): State<PostService>,
) -> Result<impl IntoResponse> {
    Ok(Json(post_service.get_posts_for_user(user.id).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostRequest {
    content: String,
}