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
    pub distribute_min: u64,
    pub distribute_target: u64,
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let redis_url = std::env::var("REDIS_URL").expect("missing REDIS_URL");
    let solana_rpc_url = std::env::var("SOLANA_RPC_URL").expect("missing SOLANA_RPC_URL");

    let access_token_secret =
        std::env::var("ACCESS_TOKEN_SECRET").expect("missing ACCESS_TOKEN_SECRET");
    let renew_token_secret =
        std::env::var("RENEW_TOKEN_SECRET").expect("missing RENEW_TOKEN_SECRET");

    let base_rpc_url = std::env::var("BASE_RPC_URL")
        .expect("missing ")
        .parse()
        .expect("invalid BASE_RPC_URL");
    let sepolia_rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .expect("missing ")
        .parse()
        .expect("invalid SEPOLIA_RPC_URL");

    let operator_pk = std::env::var("OPERATOR_PK").expect("missing OPERATOR_PK");

    let distribute_min = std::env::var("DISTRIBUTE_MIN")
        .expect("missing DISTRIBUTE_MIN")
        .parse()
        .expect("invalid DISTRIBUTE_MIN");
    let distribute_target = std::env::var("DISTRIBUTE_TARGET")
        .expect("missing DISTRIBUTE_TARGET")
        .parse()
        .expect("invalid DISTRIBUTE_TARGET");

    Env {
        access_token_secret,
        db_url,
        redis_url,
        renew_token_secret,
        solana_rpc_url,
        base_rpc_url,
        sepolia_rpc_url,
        operator_pk,
        distribute_min,
        distribute_target,
    }
});
