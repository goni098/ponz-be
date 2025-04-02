use axum::http::StatusCode;
use axum_derive_error::ErrorResponse;

use std::borrow::Cow;

#[allow(dead_code)]
#[derive(ErrorResponse, thiserror::Error)]
pub enum ServerErr {
    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    PathRejection(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    FormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    BodyRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("{0:#?}")]
    #[status(StatusCode::BAD_REQUEST)]
    BadRequest(Cow<'static, str>),

    #[error("{0:#?}")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized(Cow<'static, str>),

    #[error("{0:#?}")]
    Internal(Cow<'static, str>),

    #[error("{0}")]
    EnvError(Cow<'static, str>),

    #[error("{0:#?}")]
    Custom(Cow<'static, str>),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Database(#[from] database::sea_orm::error::DbErr),

    #[error(transparent)]
    Shared(#[from] shared::SharedErr),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),
}

pub type ServerRlt<A> = Result<A, ServerErr>;
