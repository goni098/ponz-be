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

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub id: i64,
    pub telegram_id: i64,
}

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = ServerErr;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> ServerRlt<Self> {
        let secret = shared::env::read_env("ACCESS_TOKEN_SECRET")?;

        parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Unauthorized("Missing Authorization".into()))
            .and_then(|bearer| decode_token(bearer.token(), secret))
            .map(Self)
    }
}

pub fn decode_token(token: &str, secret: String) -> ServerRlt<Claims> {
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
