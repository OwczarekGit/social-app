use axum::extract::{Path, State};
use axum::routing::{delete, get, put};
use axum::{response::IntoResponse, routing::post, Json, Router};
use dto::post::*;

use crate::app_state::ActiveUser;
use crate::service::post::{Post, PostService};
use crate::AppState;

use crate::service::domain::ImageDomain;
use crate::SysRes;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_posts))
        .route("/:id", get(get_user_posts))
        .route("/create", post(create_post))
        .route("/edit/:id", put(edit_post))
        .route("/delete/:id", delete(delete_post))
}

pub async fn create_post(
    user: ActiveUser,
    State(post_service): State<PostService>,
    Json(request): Json<CreatePostRequest>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        post_service.create_post(user.id, &request.content).await?,
    ))
}

pub async fn get_user_posts(
    image_domain: ImageDomain,
    State(post_service): State<PostService>,
    Path(id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        post_service
            .get_posts_for_user(id)
            .await?
            .into_iter()
            .map(|p| Post {
                author_picture_url: format!("{}{}", image_domain.0, p.author_picture_url),
                ..p
            })
            .collect::<Vec<_>>(),
    ))
}

pub async fn get_my_posts(
    image_domain: ImageDomain,
    user: ActiveUser,
    State(post_service): State<PostService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        post_service
            .get_posts_for_user(user.id)
            .await?
            .into_iter()
            .map(|p| Post {
                author_picture_url: format!("{}{}", image_domain.0, p.author_picture_url),
                ..p
            })
            .collect::<Vec<_>>(),
    ))
}

pub async fn edit_post(
    user: ActiveUser,
    State(post_service): State<PostService>,
    Path(id): Path<i64>,
    Json(request): Json<EditPostRequest>,
) -> SysRes<impl IntoResponse> {
    post_service.edit_post(user.id, id, &request.content).await
}

pub async fn delete_post(
    user: ActiveUser,
    State(post_service): State<PostService>,
    Path(id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    post_service.delete_post(user.id, id).await
}
