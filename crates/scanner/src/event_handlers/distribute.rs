use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::Utc;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait, prelude::DateTimeWithTimeZone},
};
use shared::AppResult;
use web3::contracts::router::Router::DistributeUserFund;

pub async fn handle_distribute_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: DistributeUserFund,
) -> AppResult<()> {
    let args = serde_json::json!({
        "strategyAddress": event.strategyAddress.to_string(),
        "depositor": event.depositor.to_string(),
        "depositedTokenAddress": event.depositedTokenAddress.to_string(),
        "amount": event.amount.to_string(),
        "swapContract": event.swapContract.to_string(),
    });

    let created_at = DateTimeWithTimeZone::from(Utc::now());

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Distribute,
        contract_address,
        args,
        chain,
        tx_hash,
        created_at,
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
        created_at,
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
