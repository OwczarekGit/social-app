use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

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
    AccountActivationWrongPassword,

    // Database errors
    DatabaseConnectionError,
    DatabaseOperationError,
    DatabaseQueryError,
    RedisError,
    JsonParseError,
    Neo4jIOError,
    Neo4jNodeNotFound,
    Neo4jInvalidNode(i64),
    Neo4jQueryError,

    // Generic errors
    InternalServerError,
    BadRequest,
    NotFound,

    // Chat errors
    InvalidSendMessageToFriendRequest(i64, i64),
    FriendMessageMissingUserNode,
    FriendMessageMissingMessageBody,

    // Relation errors
    RelationErrorIsAlreadyFriend(i64, i64),
    RelationAttemptToAddSelfAsFriend(i64),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // let mut res = (axum::http::StatusCode::INTERNAL_SERVER_ERROR).into_response();
        let code = match self {
            Error::LoginError => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::UnauthorizedForAdminOperations(_) => StatusCode::UNAUTHORIZED,
            Error::UnauthorizedForUserOperations => StatusCode::UNAUTHORIZED,
            Error::EmailTaken => StatusCode::BAD_REQUEST,
            Error::EmailTakenPendingActivation => StatusCode::BAD_REQUEST,
            Error::InvalidSendMessageToFriendRequest(_,_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let mut res = code.into_response();

        res.extensions_mut().insert(self);
        res
    }
}

impl From<redis::RedisError> for Error{
    fn from(_: redis::RedisError) -> Self {
        Self::RedisError
    }
}

impl From<axum::http::StatusCode> for Error {
    fn from(value: axum::http::StatusCode) -> Self {
        match value {
            axum::http::StatusCode::UNAUTHORIZED => Self::Unauthorized,
            axum::http::StatusCode::NOT_FOUND => Self::NotFound,
            axum::http::StatusCode::BAD_REQUEST => Self::BadRequest,
            axum::http::StatusCode::FORBIDDEN => Self::BadRequest,
            _ => Self::UnhandledError,
        }
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        match value {
            sea_orm::DbErr::Conn(_) => Self::DatabaseConnectionError,
            sea_orm::DbErr::Exec(_) => Self::DatabaseOperationError,
            sea_orm::DbErr::Query(_) => Self::DatabaseQueryError,
            sea_orm::DbErr::Json(_) => Self::JsonParseError,
            // DbErr::ConnectionAcquire(_) => {}
            // DbErr::TryIntoErr { .. } => {}
            // DbErr::ConvertFromU64(_) => {}
            // DbErr::UnpackInsertId => {}
            // DbErr::UpdateGetPrimaryKey => {}
            // DbErr::RecordNotFound(_) => {}
            // DbErr::AttrNotSet(_) => {}
            // DbErr::Custom(_) => {}
            // DbErr::Type(_) => {}
            // DbErr::Migration(_) => {}
            // DbErr::RecordNotInserted => {}
            // DbErr::RecordNotUpdated => {}
            _ => Self::UnhandledError
        }
    }
}

impl From<neo4rs::Error> for Error {
    fn from(value: neo4rs::Error) -> Self {
        match value {
            neo4rs::Error::IOError { .. } => Self::Neo4jIOError,
        //     neo4rs::Error::UrlParseError(_) => {}
        //     neo4rs::Error::UnsupportedScheme(_) => {}
        //     neo4rs::Error::InvalidDnsName(_) => {}
        //     neo4rs::Error::ConnectionError => {}
        //     neo4rs::Error::StringTooLong => {}
        //     neo4rs::Error::MapTooBig => {}
        //     neo4rs::Error::BytesTooBig => {}
        //     neo4rs::Error::ListTooLong => {}
        //     neo4rs::Error::InvalidConfig => {}
        //     neo4rs::Error::UnsupportedVersion(_) => {}
        //     neo4rs::Error::UnexpectedMessage(_) => {}
        //     neo4rs::Error::UnknownType(_) => {}
        //     neo4rs::Error::UnknownMessage(_) => {}
        //     neo4rs::Error::ConversionError => {}
        //     neo4rs::Error::AuthenticationError(_) => {}
        //     neo4rs::Error::InvalidTypeMarker(_) => {}
        //     neo4rs::Error::DeserializationError(_) => {}
            _ => Self::UnhandledError,
        }
    }
}