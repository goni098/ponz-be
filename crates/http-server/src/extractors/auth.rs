use crate::error::{
    ServerErr::{self, *},
    ServerRlt,
};
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use shared::env::ENV;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub id: i64,
    pub address: String,
}

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = ServerErr;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> ServerRlt<Self> {
        parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Unauthorized("Missing Authorization".into()))
            .and_then(|bearer| decode_token(bearer.token(), &ENV.access_token_secret))
            .map(Self)
    }
}

pub fn decode_token(token: &str, secret: &str) -> ServerRlt<Claims> {
    jsonwebtoken::decode::<Claims>(
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
