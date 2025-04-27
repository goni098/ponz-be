use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::DateTime;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait},
};
use shared::{AppError, AppResult};
use web3::contracts::strategy::Strategy::ClaimRewardStrategy;

pub async fn handle_claim_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: ClaimRewardStrategy,
) -> AppResult<()> {
    let args = serde_json::json!({
        "actualDepositAmount": event.withdrawer.to_string(),
        "receiver": event.receiver.to_string(),
        "depositAmount": event.token.to_string(),
        "tokenAddress": event.claimedAt.to_string(),
    });

    let claimed_at = DateTime::from_timestamp(event.claimedAt.to(), 0)
        .ok_or(AppError::Custom("Invalid claimedAt timestamp".into()))?;

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Claim,
        contract_address,
        args,
        chain,
        tx_hash,
        claimed_at.into(),
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
