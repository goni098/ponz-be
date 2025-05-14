mod rebalance_on_deadline;

use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
pub use rebalance_on_deadline::*;
use shared::AppResult;
use web3::client::WalletClient;

pub async fn process(
    chain: NamedChain,
    wallet_client: &WalletClient,
    db: &DatabaseConnection,
) -> AppResult<()> {
    Ok(())
}
