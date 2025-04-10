use std::str::FromStr;

use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use database::{repositories::user, sea_orm::DatabaseConnection};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use validator::Validate;

use crate::{
    error::{ServerErr, ServerRlt},
    extractors::{auth::Claims, validation::ValidatedPayload, validation::is_valid_pubkey},
};

#[derive(Deserialize, Validate)]
pub struct Payload {
    message: String,
    signed_message: String,
    #[validate(custom(function = "is_valid_pubkey"))]
    address: String,
}

#[derive(Serialize)]
pub struct Token {
    access_token: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    ValidatedPayload(payload): ValidatedPayload<Payload>,
) -> ServerRlt<Json<Token>> {
    let Payload {
        address,
        message,
        signed_message,
    } = payload;

    let sig = hex::decode(signed_message)
        .map_err(|_| ServerErr::Unauthorized("invalid signed message".into()))?;

    let pubkey =
        Pubkey::from_str(&address).map_err(|e| ServerErr::BadRequest(e.to_string().into()))?;

    let valid_signature =
        nacl::sign::verify(sig.as_slice(), message.as_bytes(), &pubkey.to_bytes())
            .map_err(|e| ServerErr::Unauthorized(e.message.into()))?;

    if !valid_signature {
        return Err(ServerErr::Unauthorized("invalid signature".into()));
    }

    let user_id = user::create_if_not_exist(&db, pubkey.to_string()).await?;

    let token = sign(user_id, address)?;

    Ok(Json(token))
}

fn sign(user_id: i64, address: String) -> ServerRlt<Token> {
    let secret = shared::env::read_env("ACCESS_TOKEN_SECRET")?;

    let header = Header::new(Algorithm::HS256);

    let secret_key = EncodingKey::from_secret(secret.as_bytes());

    let access_exp = Utc::now()
        .checked_add_signed(Duration::days(3))
        .ok_or(ServerErr::Internal(
            "get none from checked_add_signed()".into(),
        ))?
        .timestamp() as u32;

    let claims = Claims {
        exp: access_exp,
        id: user_id,
        address,
    };

    let access_token = jsonwebtoken::encode(&header, &claims, &secret_key)
        .map_err(|e| ServerErr::Unauthorized(e.to_string().into()))?;

    Ok(Token { access_token })
}
