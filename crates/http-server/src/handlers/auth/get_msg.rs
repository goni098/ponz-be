use axum::Json;
use rand::{Rng, distr::Alphanumeric};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::HttpResult,
    extractors::{
        state::Redis,
        validation::{ValidatedParams, is_evm_address},
    },
};

#[derive(Deserialize, Validate)]
pub struct Params {
    #[validate(custom(function = "is_evm_address"))]
    address: String,
}

#[derive(Serialize)]
pub struct Message {
    msg: String,
}

pub async fn handler(
    Redis(mut redis): Redis,
    ValidatedParams(params): ValidatedParams<Params>,
) -> HttpResult<Json<Message>> {
    let msg = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    redis
        .set::<String, &String, ()>(params.address, &msg)
        .await?;

    Ok(Json(Message { msg }))
}
