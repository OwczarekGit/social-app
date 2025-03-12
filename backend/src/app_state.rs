use crate::service::account::AccountService;
use crate::service::activation::ActivationService;
use crate::service::chat::ChatService;
use crate::service::domain::DomainService;
use crate::service::email::EmailService;
use crate::service::friend::FriendService;
use crate::service::image::ImageService;
use crate::service::notification::NotificationService;
use crate::service::post::PostService;
use crate::service::profile::ProfileService;
use crate::service::tag::TagService;
use crate::service::wallpaper::WallpaperService;
use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_macros::FromRef;
use minio_rsc::Minio;
use neo4rs::Graph;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub account_service: AccountService,
    pub email_service: EmailService,
    pub post_service: PostService,
    pub notification_service: NotificationService,
    pub friend_service: FriendService,
    pub profile_service: ProfileService,
    pub image_service: ImageService,
    pub tag_service: TagService,
    pub chat_service: ChatService,
    pub wallpaper_service: WallpaperService,
    pub activation_service: ActivationService,
    pub domain_service: DomainService,
}

impl AppState {
    pub async fn new(
        redis_connection: ConnectionManager,
        postgres_connection: DatabaseConnection,
        neo4j_connection: Arc<Graph>,
        minio_connection: Minio,
    ) -> Self {
        Self {
            account_service: AccountService::new(
                redis_connection,
                postgres_connection.clone(),
                neo4j_connection.clone(),
            ),
            email_service: EmailService::new(),
            post_service: PostService::new(neo4j_connection.clone()),
            notification_service: NotificationService::new(
                neo4j_connection.clone(),
                postgres_connection.clone(),
            ),
            friend_service: FriendService::new(neo4j_connection.clone()),
            profile_service: ProfileService::new(
                neo4j_connection.clone(),
                minio_connection.clone(),
            ),
            image_service: ImageService::new(neo4j_connection.clone(), minio_connection.clone()),
            tag_service: TagService::new(neo4j_connection.clone()),
            chat_service: ChatService::new(neo4j_connection.clone()),
            wallpaper_service: WallpaperService::new(neo4j_connection.clone()),
            activation_service: ActivationService::new(postgres_connection.clone()),
            domain_service: DomainService::new(postgres_connection.clone()).await,
        }
    }
}

impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = crate::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
