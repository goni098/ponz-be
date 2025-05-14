use alloy::{rpc::types::Log, sol_types::SolEvent};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use serde_json::json;
use shared::{AppError, AppResult};
use web3::contracts::cross_chain_router::CrossChainRouter::TransferFundCrossChain;

use super::Context;

pub async fn process(
    db: &DatabaseConnection,
    chain: NamedChain,
    log: Log<TransferFundCrossChain>,
    context: Context,
) -> AppResult<()> {
    let contract_address = log.address();
    let log_index = log.log_index.unwrap_or_default();
    let tx_hash = log.transaction_hash.unwrap_or_default();
    let event = log.inner.data;

    let emit_at = DateTime::from_timestamp(event.transferFundCrossChainAt.to::<i64>(), 0).ok_or(
        AppError::Custom(
            "Invalid TransferFundCrossChain transferFundCrossChainAt timestamp".into(),
        ),
    )?;

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        TransferFundCrossChain::SIGNATURE,
        contract_address,
        json!(event),
        chain,
        tx_hash,
        log_index,
        emit_at.into(),
        context.is_scanner(),
    )
    .await?;

    repositories::transfer_fund_cross_chain_event::upsert(&db_tx, chain, tx_hash, log_index, event)
        .await?;

    db_tx.commit().await?;

    Ok(())
}
