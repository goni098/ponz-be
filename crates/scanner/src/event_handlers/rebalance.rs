use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::{EventArgs, contracts::router::Router::RebalanceFundSameChain};

pub async fn handle_rebalance_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    log_index: u64,
    chain: NamedChain,
    event: RebalanceFundSameChain,
    block_timestamp: u64,
) -> AppResult<()> {
    let created_at = DateTime::from_timestamp(block_timestamp as i64, 0)
        .ok_or(AppError::Custom("Invalid block_timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::upsert(
        &db_tx,
        ContractEventName::Rebalance,
        contract_address,
        event.json_args(),
        chain,
        tx_hash,
        log_index,
        created_at.into(),
    )
    .await?;

    repositories::rebalance_txn::upsert(&db_tx, chain, tx_hash, log_index, event).await?;

    db_tx.commit().await?;

    Ok(())
}
