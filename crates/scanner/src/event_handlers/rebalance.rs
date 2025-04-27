use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::Utc;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait, prelude::DateTimeWithTimeZone},
};
use shared::AppResult;
use web3::contracts::router::Router::RebalanceFundSameChain;

pub async fn handle_rebalance_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: RebalanceFundSameChain,
) -> AppResult<()> {
    let args = serde_json::json!({
        "strategyAddress": event.strategyAddress.to_string(),
        "userAddress": event.userAddress.to_string(),
        "underlyingAsset": event.underlyingAsset.to_string(),
        "receivedAmount": event.receivedAmount.to_string(),
        "receivedReward": event.receivedReward.to_string(),
    });

    let created_at = DateTimeWithTimeZone::from(Utc::now());

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Rebalance,
        contract_address,
        args,
        chain,
        tx_hash,
        created_at,
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
        created_at,
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
