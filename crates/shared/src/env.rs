use std::sync::LazyLock;

use url::Url;

pub struct Env {
    pub db_url: String,
    pub redis_url: String,
    pub access_token_secret: String,
    pub renew_token_secret: String,

    pub base_rpc_url: Url,
    pub arbitrum_rpc_url: Url,
    pub sepolia_rpc_url: Url,
    pub arb_sepolia_rpc_url: Url,

    pub base_ws_url: Url,
    pub arbitrum_ws_url: Url,
    pub sepolia_ws_url: Url,
    pub arb_sepolia_ws_url: Url,

    pub distribute_min: u64,
    pub distribute_target: u64,
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let redis_url = std::env::var("REDIS_URL").expect("missing REDIS_URL");

    let access_token_secret =
        std::env::var("ACCESS_TOKEN_SECRET").expect("missing ACCESS_TOKEN_SECRET");

    let renew_token_secret =
        std::env::var("RENEW_TOKEN_SECRET").expect("missing RENEW_TOKEN_SECRET");

    let base_rpc_url = std::env::var("BASE_RPC_URL")
        .expect("missing BASE_RPC_URL")
        .parse()
        .expect("invalid BASE_RPC_URL");

    let arbitrum_rpc_url = std::env::var("ARBITRUM_RPC_URL")
        .expect("missing ARBITRUM_RPC_URL")
        .parse()
        .expect("invalid ARBITRUM_RPC_URL");

    let sepolia_rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .expect("missing SEPOLIA_RPC_URL")
        .parse()
        .expect("invalid SEPOLIA_RPC_URL");

    let arb_sepolia_rpc_url = std::env::var("ARB_SEPOLIA_RPC_URL")
        .expect("missing ARB_SEPOLIA_RPC_URL")
        .parse()
        .expect("invalid ARB_SEPOLIA_RPC_URL");

    let base_ws_url = std::env::var("BASE_WS_URL")
        .expect("missing BASE_WS_URL")
        .parse()
        .expect("invalid BASE_WS_URL");

    let arbitrum_ws_url = std::env::var("ARBITRUM_WS_URL")
        .expect("missing ARBITRUM_WS_URL")
        .parse()
        .expect("invalid ARBITRUM_WS_URL");

    let sepolia_ws_url = std::env::var("SEPOLIA_WS_URL")
        .expect("missing SEPOLIA_WS_URL")
        .parse()
        .expect("invalid SEPOLIA_WS_URL");

    let arb_sepolia_ws_url = std::env::var("ARB_SEPOLIA_WS_URL")
        .expect("missing ARB_SEPOLIA_WS_URL")
        .parse()
        .expect("invalid ARB_SEPOLIA_WS_URL");

    let distribute_min = std::env::var("DISTRIBUTE_MIN")
        .expect("missing DISTRIBUTE_MIN")
        .parse()
        .expect("invalid DISTRIBUTE_MIN");

    let distribute_target = std::env::var("DISTRIBUTE_TARGET")
        .expect("missing DISTRIBUTE_TARGET")
        .parse()
        .expect("invalid DISTRIBUTE_TARGET");

    Env {
        db_url,
        redis_url,

        access_token_secret,
        renew_token_secret,

        base_rpc_url,
        sepolia_rpc_url,
        arb_sepolia_rpc_url,
        arbitrum_rpc_url,

        arb_sepolia_ws_url,
        arbitrum_ws_url,
        base_ws_url,
        sepolia_ws_url,

        distribute_min,
        distribute_target,
    }
});
