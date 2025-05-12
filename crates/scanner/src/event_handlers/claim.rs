use alloy::{
    primitives::{Address, TxHash},
    rpc::types::Log,
};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::{EventArgs, contracts::referral::Refferal::Claim};

pub async fn handle_claim_event(
    db: &DatabaseConnection,
    chain: NamedChain,
    log: Log<Claim>,
) -> AppResult<()> {
    let block_timestamp = log.block_timestamp.unwrap_or_default();

    let created_at = if let Some(timestamp) = log.block_timestamp {
         DateTime::from_timestamp(timestamp as i64, 0)
        .ok_or(AppError::Custom("Invalid block_timestamp".into()))?;
    } else {
        
    };

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        ContractEventName::Claim,
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
