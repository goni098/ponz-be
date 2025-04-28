use std::time::Duration;

use alloy_chains::NamedChain;
use shared::AppResult;
use tokio::time::sleep;

mod distribute;
mod rebalance;

pub async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    shared::logging::set_up("operator");

    tokio::spawn(async move {
        loop {
            if let Err(error) = rebalance::handler().await {
                tracing::error!("{:#?}", error);
            }
            sleep(Duration::from_secs(3600)).await;
        }
    });

    tracing::info!("🦀 starting operator on {}...", chain);

    loop {
        if let Err(error) = distribute::handler().await {
            tracing::error!("{:#?}", error);
        }
        sleep(Duration::from_secs(3600)).await;
    }
}
