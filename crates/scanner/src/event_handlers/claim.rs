use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::contracts::referral::Refferal::Claim;

pub async fn handle_claim_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    log_index: i32,
    chain: NamedChain,
    event: Claim,
    block_timestamp: u64,
) -> AppResult<()> {
    let args = serde_json::json!({
        "amount": event.amount.to_string(),
        "from": event.from.to_string(),
        "to": event.to.to_string(),
    });

    let created_at = DateTime::from_timestamp(block_timestamp as i64, 0)
        .ok_or(AppError::Custom("Invalid block_timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        ContractEventName::Claim,
        contract_address,
        args,
        chain,
        tx_hash,
        log_index,
        created_at.into(),
    )
    .await?;

    repositories::claim_txn::create(
        &db_tx,
        chain,
        event.amount,
        event.from,
        event.to,
        created_at.into(),
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
