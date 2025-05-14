use std::time::Duration;

use alloy_chains::NamedChain;
use database::sea_orm::{ConnectOptions, Database};
use operator::{distribute, rebalance, withdraw};
use pools::ExternalPoolsService;
use shared::{AppResult, env::ENV};
use web3::client::wallet_client;

#[tokio::main]
async fn main() {
    shared::logging::set_up("operator");
    let chain = shared::arg::parse_chain_arg();
    bootstrap(chain).await.unwrap();
}

async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;
    let wallet_client = wallet_client(chain);
    let pools_service = ExternalPoolsService::new();

    loop {
        let _ = distribute::process(chain, &wallet_client, &db, &pools_service).await;
        let _ = rebalance::process(chain, &wallet_client, &db).await;
        let _ = withdraw::process(chain, &wallet_client, &db).await;

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
