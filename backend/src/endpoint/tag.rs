use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{get, post, put};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::service::tag::TagService;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_tags_with_usage))
}

pub async fn get_all_tags_with_usage(
    State(tag_service): State<TagService>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(tag_service.get_all_tags_with_usage().await?))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_tag))
        .route("/:id", put(update_tag).delete(delete_tag))
}

pub async fn update_tag(
    Path(id): Path<i64>,
    State(tag_service): State<TagService>,
    Json(request): Json<UpdateTagRequest>,
) -> Result<(), StatusCode> {
    tag_service.update_tag(id, &request.name).await
}

pub async fn delete_tag(
    State(tag_service): State<TagService>,
    Path(id): Path<i64>,
) -> Result<(), StatusCode> {
    tag_service.delete_tag(id).await
}

pub async fn create_tag(
    State(tag_service): State<TagService>,
    Json(request): Json<CreateTagRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if request.name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(tag_service.create_tag(&request.name).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTagRequest {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTagRequest {
    name: String
}
