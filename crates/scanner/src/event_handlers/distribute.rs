use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::contracts::router::Router::DistributeUserFund;

pub async fn handle_distribute_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    log_index: i32,
    chain: NamedChain,
    event: DistributeUserFund,
    block_timestamp: u64,
) -> AppResult<()> {
    let args = serde_json::json!({
        "strategyAddress": event.strategyAddress.to_string(),
        "depositor": event.depositor.to_string(),
        "depositedTokenAddress": event.depositedTokenAddress.to_string(),
        "amount": event.amount.to_string(),
        "swapContract": event.swapContract.to_string(),
    });

    let created_at = DateTime::from_timestamp(block_timestamp as i64, 0)
        .ok_or(AppError::Custom("Invalid block_timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Distribute,
        contract_address,
        args,
        chain,
        tx_hash,
        log_index,
        created_at.into(),
    )
    .await?;

    repositories::distribute_txn::create(
        &db_tx,
        chain,
        event.strategyAddress,
        event.depositor,
        event.depositedTokenAddress,
        event.amount,
        event.swapContract,
        created_at.into(),
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
