mod rebalance_on_deadline;

use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
pub use rebalance_on_deadline::*;
use shared::AppResult;

pub async fn process(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    Ok(())
}
