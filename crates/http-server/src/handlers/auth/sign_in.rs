use std::str::FromStr;

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
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use validator::Validate;

use crate::{
    error::{ServerErr, ServerRlt},
    extractors::{
        auth::{Claims, Sub},
        state::Redis,
        validation::{ValidatedPayload, is_valid_pubkey},
    },
};

#[derive(Deserialize, Validate)]
pub struct Payload {
    message: String,
    signature: String,
    #[validate(custom(function = "is_valid_pubkey"))]
    address: String,
}

#[derive(Serialize)]
pub struct Tokens {
    access_token: String,
    renew_token: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Redis(mut redis): Redis,
    ValidatedPayload(payload): ValidatedPayload<Payload>,
) -> ServerRlt<Json<Tokens>> {
    let Payload {
        address,
        message,
        signature,
    } = payload;

    let stored_message = redis
        .get::<&String, Option<String>>(&address)
        .await?
        .ok_or(ServerErr::Unauthorized(
            "Message has not been created".into(),
        ))?;

    if stored_message != message {
        return Err(ServerErr::Unauthorized("Invalid messgage".into()));
    }

    let sig = Signature::from_str(&signature)
        .map_err(|_| ServerErr::Unauthorized("Invalid signature".into()))?;

    let pubkey =
        Pubkey::from_str(&address).map_err(|e| ServerErr::BadRequest(e.to_string().into()))?;

    let valid_signature = sig.verify(pubkey.as_ref(), message.as_bytes());

    if !valid_signature {
        return Err(ServerErr::Unauthorized("Wrong signature".into()));
    }

    let user_id = user::create_if_not_exist(&db, pubkey.to_string()).await?;

    let tokens = Tokens::sign_from(user_id, address)?;

    tokens.save_renew_token(user_id, &db).await?;

    Ok(Json(tokens))
}

impl Tokens {
    pub fn sign_from(user_id: i64, address: String) -> ServerRlt<Self> {
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
            .map_err(|e| ServerErr::Unauthorized(e.to_string().into()))?;

        let renew_token = jsonwebtoken::encode(&header, &sub, &renew_secret_key)
            .map_err(|e| ServerErr::Unauthorized(e.to_string().into()))?;

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
    use solana_sdk::{
        signature::Keypair,
        signer::{EncodableKey, Signer},
    };

    #[test]
    fn gen_signature() {
        let message = "abcxyz";
        let keypair = Keypair::read_from_file("/home/vitaminc/.config/solana/id.json").unwrap();
        let signature = keypair.sign_message(message.as_bytes());

        assert!(signature.verify(keypair.pubkey().as_ref(), message.as_bytes()));

        dbg!(signature);
    }
}
