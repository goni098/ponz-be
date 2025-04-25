use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Deserialize;
use shared::env::ENV;

use crate::{
    error::{HttpException, HttpResult},
    extractors::auth::{Sub, decode_token},
};

use super::sign_in::Tokens;

#[derive(Deserialize)]
pub struct Payload {
    renew_token: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<Payload>,
) -> HttpResult<Json<Tokens>> {
    let Payload { renew_token } = payload;

    let claims = decode_token::<Sub>(&renew_token, &ENV.renew_token_secret)?;

    let token = repositories::renew_token::find_by_user_id(&db, claims.sub)
        .await?
        .ok_or(HttpException::Unauthorized("Token not found".into()))?;

    if token != renew_token {
        return Err(HttpException::Unauthorized("Wrong token".into()));
    }

    let user = repositories::user::find_by_id(&db, claims.sub)
        .await?
        .ok_or(HttpException::Internal("User not found".into()))?;

    let tokens = Tokens::sign_from(user.id, user.address)?;

    tokens.save_renew_token(user.id, &db).await?;

    Ok(Json(tokens))
}
