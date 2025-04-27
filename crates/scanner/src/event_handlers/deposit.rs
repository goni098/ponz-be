use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::contracts::router::Router::DepositFund;

pub async fn handle_deposit_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: DepositFund,
) -> AppResult<()> {
    let args = serde_json::json!({
        "receiver": event.receiver.to_string(),
        "tokenAddress": event.tokenAddress.to_string(),
        "depositAmount": event.depositAmount.to_string(),
        "actualDepositAmount": event.actualDepositAmount.to_string(),
        "depositedAt": event.depositedAt.to_string(),
    });

    let deposited_at = DateTime::from_timestamp(event.depositedAt.to(), 0)
        .ok_or(AppError::Custom("Invalid depositedAt timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Deposit,
        contract_address,
        args,
        chain,
        tx_hash,
        deposited_at.into(),
    )
    .await?;

    repositories::deposit_txn::create(
        &db_tx,
        chain,
        event.receiver,
        event.tokenAddress,
        event.depositAmount,
        event.actualDepositAmount,
        deposited_at.into(),
        deposited_at.into(),
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
