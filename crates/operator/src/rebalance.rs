mod rebalance_on_deadline;

use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
pub use rebalance_on_deadline::*;
use shared::AppResult;
use web3::client::WalletClient;

pub async fn process_from_db(
    chain: NamedChain,
    wallet_client: &WalletClient,
    db: &DatabaseConnection,
) -> AppResult<()> {
    let snapshots =
        repositories::distribute_user_fund_event::find_unrebalanced_and_order_than_10days(db, 10)
            .await?;

    for snapshot in snapshots {
        let tx_hash = snapshot.tx_hash.clone();
        let log_index = snapshot.log_index as u64;

        match rebalance_on_deadline(chain, wallet_client, snapshot).await {
            Ok(_) => {
                repositories::distribute_user_fund_event::pin_as_resolved(db, tx_hash, log_index)
                    .await?;
            }
            Err(error) => {
                repositories::distribute_user_fund_event::pin_as_failed(
                    db,
                    tx_hash,
                    log_index,
                    format!("{:#?}", error),
                )
                .await?;
            }
        }
    }

    Ok(())
}
