use std::sync::LazyLock;

use url::Url;

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
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let redis_url = std::env::var("REDIS_URL").unwrap();
    let solana_rpc_url = std::env::var("SOLANA_RPC_URL").unwrap();
    let access_token_secret = std::env::var("ACCESS_TOKEN_SECRET").unwrap();
    let renew_token_secret = std::env::var("RENEW_TOKEN_SECRET").unwrap();
    let base_rpc_url = std::env::var("BASE_RPC_URL").unwrap().parse().unwrap();
    let sepolia_rpc_url = std::env::var("SEPOLIA_RPC_URL").unwrap().parse().unwrap();
    let operator_pk = std::env::var("OPERATOR_PK").unwrap();

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
