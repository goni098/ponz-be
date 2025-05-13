mod distribute_when_deposit;
mod distribute_when_rebalance;

use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
pub use distribute_when_deposit::*;
pub use distribute_when_rebalance::*;
use shared::AppResult;

pub async fn process(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    Ok(())
}
