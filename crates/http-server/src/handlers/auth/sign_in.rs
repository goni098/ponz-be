use std::str::FromStr;

use alloy::signers::Signature;
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use database::{
    repositories::{self, user},
    sea_orm::{DatabaseConnection, DbErr},
};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use shared::env::ENV;

use crate::{
    error::{HttpException, HttpResult},
    extractors::{
        auth::{Claims, Sub},
        state::Redis,
    },
};

#[derive(Deserialize)]
pub struct Payload {
    message: String,
    signature: String,
}

#[derive(Serialize)]
pub struct Tokens {
    access_token: String,
    renew_token: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Redis(mut redis): Redis,
    Json(payload): Json<Payload>,
) -> HttpResult<Json<Tokens>> {
    let Payload { message, signature } = payload;

    let signature = Signature::from_str(&signature)
        .map_err(|_| HttpException::Unauthorized("Invalid signature".into()))?;

    let address = signature
        .recover_address_from_msg(&message)
        .map_err(|_| HttpException::Unauthorized("Wrong signature".into()))?;

    let stored_message = redis
        .get::<&String, Option<String>>(&address.to_string())
        .await?
        .ok_or(HttpException::Unauthorized(
            "Message has not been created".into(),
        ))?;

    if stored_message != message {
        return Err(HttpException::Unauthorized("Invalid messgage".into()));
    }

    let user_id = user::create_if_not_exist(&db, address).await?;

    let tokens = Tokens::sign_from(user_id, address.to_string())?;

    tokens.save_renew_token(user_id, &db).await?;

    Ok(Json(tokens))
}

impl Tokens {
    pub fn sign_from(user_id: i64, address: String) -> HttpResult<Self> {
        let header = Header::new(Algorithm::HS256);

        let access_secret_key = EncodingKey::from_secret(ENV.access_token_secret.as_bytes());
        let renew_secret_key = EncodingKey::from_secret(ENV.renew_token_secret.as_bytes());

        let now = Utc::now().timestamp();

        let access_exp = now + Duration::days(3).num_seconds();
        let renew_exp = access_exp + Duration::days(90).num_seconds();

        let claims = Claims {
            exp: access_exp as u32,
            id: user_id,
            address,
        };

        let sub = Sub {
            exp: renew_exp as u32,
            sub: user_id,
        };

        let access_token = jsonwebtoken::encode(&header, &claims, &access_secret_key)
            .map_err(|e| HttpException::Unauthorized(e.to_string().into()))?;

        let renew_token = jsonwebtoken::encode(&header, &sub, &renew_secret_key)
            .map_err(|e| HttpException::Unauthorized(e.to_string().into()))?;

        Ok(Self {
            access_token,
            renew_token,
        })
    }

    pub async fn save_renew_token(
        &self,
        user_id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        repositories::renew_token::create_overwrite(db, user_id, self.renew_token.clone()).await
    }
}

#[cfg(test)]
mod test {
    use alloy::signers::{SignerSync, local::PrivateKeySigner};

    #[test]
    fn gen_signature() {
        let message = "abcxyz";
        let signer = PrivateKeySigner::random();

        let signature = signer.sign_message_sync(message.as_bytes()).unwrap();

        let recovered = signature.recover_address_from_msg(message).unwrap();

        assert_eq!(recovered, signer.address());

        dbg!(signature.to_string());
    }
}
