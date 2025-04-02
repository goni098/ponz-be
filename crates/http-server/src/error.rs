use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use std::borrow::Cow;

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum ServerErr {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    PathRejection(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    FormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    BodyRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("{0:#?}")]
    BadRequest(Cow<'static, str>),

    #[error("{0:#?}")]
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

impl IntoResponse for ServerErr {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, format!("{:#?}", err)),
            Self::PathRejection(err) => (StatusCode::BAD_REQUEST, format!("{:#?}", err)),
            Self::FormRejection(err) => (StatusCode::BAD_REQUEST, format!("{:#?}", err)),
            Self::QueryRejection(err) => (StatusCode::BAD_REQUEST, format!("{:#?}", err)),
            Self::BodyRejection(err) => (StatusCode::BAD_REQUEST, format!("{:#?}", err)),
            Self::Unauthorized(err) => (StatusCode::UNAUTHORIZED, format!("{:#?}", err)),
            _ => {
                tracing::error!("{:#?} ", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "code": status_code.as_u16(),
            "message": message,
        }));

        axum::response::IntoResponse::into_response((status_code, body))
    }
}
