use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

macro_rules! map_err {
    ($from:ty, $to:ident) => {
        impl From<$from> for Error {
            fn from(value: $from) -> Self {
                Self::$to(value)
            }
        }
    };
}

pub type SysRes<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnhandledError,

    // Authentication errors
    LoginError,
    UnauthorizedForAdminOperations(i64),
    UnauthorizedForUserOperations,
    Unauthorized,

    // Account errors
    EmailTaken,
    EmailTakenPendingActivation,
    NonExistentAccountActivationAttempt,
    AccountActivationWrongActivationKey,
    AccountForUpdatePasswordNotFound(i64),
    AccountForUpdatePasswordWrongPasswordProvided(i64),

    // Database errors
    DatabaseConnectionError,
    DatabaseOperationError,
    DatabaseQueryError,
    JsonParseError,
    Neo4jIOError,
    Neo4jNodeNotFound,
    Neo4jInvalidNode(i64),
    Neo4jQueryError,
    SeaOrm(sea_orm::DbErr),
    Redis(redis::RedisError),
    Neo4rs(neo4rs::Error),
    Neo4rsDe(neo4rs::DeError),

    // Generic errors
    RequiredEnvMissing(String, String),
    InternalServerError,
    BadRequest,
    NotFound,
    Io(std::io::Error),
    CookiesMissing,
    ParseInt(std::num::ParseIntError),

    // Chat errors
    InvalidSendMessageToFriendRequest(i64, i64),
    FriendMessageMissingUserNode,
    FriendMessageMissingMessageBody,

    // Relation errors
    RelationErrorIsAlreadyFriend(i64, i64),
    RelationAttemptToAddSelfAsFriend(i64),

    // Image errors
    UnhandlerImageProcessingError,
    Image(image::ImageError),

    // Minio errors
    Minio(minio_rsc::error::Error),
    UnhandledMinioError,
    UnauthorizedForEditingPost(i64, i64),
    AppStateMissing,
    UnauthorizedForModeratorOperations(i64),
    ImageCreationFailed,
    ProfileFetchError(i64),
    Neo4rsMissingParam(String),
}

map_err!(redis::RedisError, Redis);
map_err!(neo4rs::Error, Neo4rs);
map_err!(neo4rs::DeError, Neo4rsDe);
map_err!(std::io::Error, Io);
map_err!(image::ImageError, Image);
map_err!(minio_rsc::error::Error, Minio);
map_err!(sea_orm::DbErr, SeaOrm);
map_err!(std::num::ParseIntError, ParseInt);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let code = match self {
            Error::LoginError => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::UnauthorizedForAdminOperations(_) => StatusCode::UNAUTHORIZED,
            Error::UnauthorizedForUserOperations => StatusCode::UNAUTHORIZED,
            Error::EmailTaken => StatusCode::BAD_REQUEST,
            Error::EmailTakenPendingActivation => StatusCode::BAD_REQUEST,
            Error::InvalidSendMessageToFriendRequest(_, _) => StatusCode::BAD_REQUEST,
            Error::AccountForUpdatePasswordWrongPasswordProvided(_) => StatusCode::BAD_REQUEST,
            Error::AccountActivationWrongActivationKey => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let mut res = code.into_response();

        res.extensions_mut().insert(Arc::new(self));
        res
    }
}
