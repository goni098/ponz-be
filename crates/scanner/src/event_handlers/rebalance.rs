use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::contracts::router::Router::RebalanceFundSameChain;

pub async fn handle_rebalance_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    log_index: i32,
    chain: NamedChain,
    event: RebalanceFundSameChain,
    block_timestamp: u64,
) -> AppResult<()> {
    let args = serde_json::json!({
        "strategyAddress": event.strategyAddress.to_string(),
        "userAddress": event.userAddress.to_string(),
        "underlyingAsset": event.underlyingAsset.to_string(),
        "receivedAmount": event.receivedAmount.to_string(),
        "receivedReward": event.receivedReward.to_string(),
    });

    let created_at = DateTime::from_timestamp(block_timestamp as i64, 0)
        .ok_or(AppError::Custom("Invalid block_timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        ContractEventName::Rebalance,
        contract_address,
        args,
        chain,
        tx_hash,
        log_index,
        created_at.into(),
    )
    .await?;

    repositories::rebalance_txn::create(
        &db_tx,
        chain,
        event.strategyAddress,
        event.userAddress,
        event.underlyingAsset,
        event.receivedAmount,
        event.receivedReward,
        created_at.into(),
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
