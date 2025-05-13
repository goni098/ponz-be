use alloy::{rpc::types::Log, sol_types::SolEvent};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::{EventArgs, contracts::referral::Refferal::Claim};

use super::Context;

pub async fn handle_claim_event(
    db: &DatabaseConnection,
    chain: NamedChain,
    log: Log<Claim>,
    context: Context,
) -> AppResult<()> {
    let contract_address = log.address();
    let log_index = log.log_index.unwrap_or_default();
    let tx_hash = log.transaction_hash.unwrap_or_default();
    let event = log.inner.data;

    let created_at = DateTime::from_timestamp(event.claimAt.to::<i64>(), 0)
        .ok_or(AppError::Custom("Invalid claimAt timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        Claim::SIGNATURE,
        contract_address,
        event.json_args(),
        chain,
        tx_hash,
        log_index,
        created_at.into(),
    )
    .await?;

    repositories::claim_txn::upsert(&db_tx, chain, tx_hash, log_index, event).await?;

    db_tx.commit().await?;

    Ok(())
}
