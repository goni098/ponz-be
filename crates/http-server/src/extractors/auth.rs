use crate::error::{
    HttpException::{self, *},
    HttpResult,
};
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use shared::env::ENV;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub id: i64,
    pub address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sub {
    pub exp: u32,
    pub sub: i64,
}

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> HttpResult<Self> {
        parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Unauthorized("Missing Authorization".into()))
            .and_then(|bearer| decode_token(bearer.token(), &ENV.access_token_secret))
            .map(Self)
    }
}

pub fn decode_token<T: DeserializeOwned>(token: &str, secret: &str) -> HttpResult<T> {
    jsonwebtoken::decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => Unauthorized("Expired token".into()),
        _ => Unauthorized("Invalid token".into()),
    })
    .map(|token_data| token_data.claims)
}
