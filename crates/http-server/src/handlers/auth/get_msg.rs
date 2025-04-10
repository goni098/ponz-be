use axum::Json;
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::ServerRlt,
    extractors::{
        state::Redis,
        validation::{ValidatedParams, is_valid_pubkey},
    },
};

#[derive(Deserialize, Validate)]
pub struct Params {
    #[validate(custom(function = "is_valid_pubkey"))]
    address: String,
}

#[derive(Serialize)]
pub struct Message {
    msg: String,
}

pub async fn handler(
    Redis(mut redis): Redis,
    ValidatedParams(params): ValidatedParams<Params>,
) -> ServerRlt<Json<Message>> {
    let msg = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    redis::cmd("SET")
        .arg(params.address)
        .arg(&msg)
        .exec_async(&mut redis)
        .await?;

    Ok(Json(Message { msg }))
}
