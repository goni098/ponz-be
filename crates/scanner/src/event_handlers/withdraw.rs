use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::contracts::router::Router::WithDrawFundSameChain;

pub async fn handle_withdraw_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: WithDrawFundSameChain,
    block_timestamp: u64,
) -> AppResult<()> {
    let args = serde_json::json!({
        "receiver": event.receiver.to_string(),
        "user": event.user.to_string(),
        "strategyAddress": event.strategyAddress.to_string(),
        "tokenAddress": event.tokenAddress.to_string(),
        "share": event.share.to_string(),
    });
    let created_at = DateTime::from_timestamp(block_timestamp as i64, 0)
        .ok_or(AppError::Custom("Invalid block_timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Withdraw,
        contract_address,
        args,
        chain,
        tx_hash,
        created_at.into(),
    )
    .await?;

    repositories::withdraw_txn::create(
        &db_tx,
        chain,
        event.receiver,
        event.user,
        event.strategyAddress,
        event.tokenAddress,
        event.share,
        created_at.into(),
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
