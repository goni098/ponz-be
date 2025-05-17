use std::time::Duration;

use alloy_chains::NamedChain;
use database::sea_orm::{ConnectOptions, Database};
use operator::{distribute, rebalance, withdraw};
use pools::ExternalPoolsService;
use shared::{AppResult, env::ENV};

#[tokio::main]
async fn main() {
    shared::logging::set_up(["operator"]);
    let chain = shared::arg::parse_chain_arg();
    bootstrap(chain).await.unwrap();
}

async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;
    let pools_service = ExternalPoolsService::new(db.clone());

    loop {
        distribute::process_from_db(chain, &db, &pools_service)
            .await
            .unwrap_or_else(|error| {
                tracing::error!("process distribute snapshots error {:#?}", error)
            });

        rebalance::process_from_db(chain, &db)
            .await
            .unwrap_or_else(|error| {
                tracing::error!("process rebalance snapshots error {:#?}", error)
            });

        withdraw::process_from_db(chain, &db)
            .await
            .unwrap_or_else(|error| {
                tracing::error!("process withdraw snapshots error {:#?}", error)
            });

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
