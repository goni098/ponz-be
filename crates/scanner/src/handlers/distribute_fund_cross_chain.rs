use alloy::{rpc::types::Log, sol_types::SolEvent};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use serde_json::json;
use shared::{AppError, AppResult};
use web3::contracts::lz_executor::LzExecutor::DistributeFundCrossChain;

use super::Context;

pub async fn process(
    db: &DatabaseConnection,
    chain: NamedChain,
    log: Log<DistributeFundCrossChain>,
    context: Context,
) -> AppResult<()> {
    let contract_address = log.address();
    let log_index = log.log_index.unwrap_or_default();
    let tx_hash = log.transaction_hash.unwrap_or_default();
    let event = log.inner.data;

    let emit_at = DateTime::from_timestamp(event.distributedAt.to::<i64>(), 0).ok_or(
        AppError::Custom("Invalid DistributeFundCrossChain distributedAt timestamp".into()),
    )?;

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        DistributeFundCrossChain::SIGNATURE,
        contract_address,
        json!(event),
        chain,
        tx_hash,
        log_index,
        emit_at.into(),
        context.is_scanner(),
    )
    .await?;

    repositories::distribute_fund_cross_chain::upsert(&db_tx, chain, tx_hash, log_index, event)
        .await?;

    db_tx.commit().await?;

    Ok(())
}
