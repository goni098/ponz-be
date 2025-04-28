use std::{env::VarError, sync::LazyLock};

use url::Url;

use crate::{AppError, AppResult};

pub struct Env {
    pub db_url: String,
    pub redis_url: String,
    pub solana_rpc_url: String,
    pub access_token_secret: String,
    pub renew_token_secret: String,
    pub base_rpc_url: Url,
    pub sepolia_rpc_url: Url,
    pub operator_pk: String,
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    let db_url = read_env("DATABASE_URL").unwrap();
    let redis_url = read_env("REDIS_URL").unwrap();
    let solana_rpc_url = read_env("SOLANA_RPC_URL").unwrap();
    let access_token_secret = read_env("ACCESS_TOKEN_SECRET").unwrap();
    let renew_token_secret = read_env("RENEW_TOKEN_SECRET").unwrap();
    let base_rpc_url = read_env("BASE_RPC_URL").unwrap().parse().unwrap();
    let sepolia_rpc_url = read_env("SEPOLIA_RPC_URL").unwrap().parse().unwrap();
    let operator_pk = read_env("OPERATOR_PK").unwrap();

    Env {
        access_token_secret,
        db_url,
        redis_url,
        renew_token_secret,
        solana_rpc_url,
        base_rpc_url,
        sepolia_rpc_url,
        operator_pk,
    }
});

fn read_env(env: &str) -> AppResult<String> {
    std::env::var(env).map_err(|error| match error {
        VarError::NotUnicode(message) => {
            AppError::EnvError(message.to_str().unwrap_or_default().to_string().into())
        }
        VarError::NotPresent => {
            AppError::EnvError(format!("Missing {} env configuration", env).into())
        }
    })
}
