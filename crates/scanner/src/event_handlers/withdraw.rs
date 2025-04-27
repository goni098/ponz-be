use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use chrono::Utc;
use database::{
    enums::ContractEventName,
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait, prelude::DateTimeWithTimeZone},
};
use shared::AppResult;
use web3::contracts::strategy::Strategy::Withdraw;

pub async fn handle_withdraw_event(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: Withdraw,
) -> AppResult<()> {
    let args = serde_json::json!({
        "sender": event.sender.to_string(),
        "receiver": event.receiver.to_string(),
        "owner": event.owner.to_string(),
        "assets": event.assets.to_string(),
        "shares": event.shares.to_string(),
    });

    let created_at = DateTimeWithTimeZone::from(Utc::now());

    let db_tx = db.begin().await?;

    repositories::contract_event::create(
        &db_tx,
        ContractEventName::Withdraw,
        contract_address,
        args,
        chain,
        tx_hash,
        created_at,
    )
    .await?;

    repositories::withdraw_txn::create(
        &db_tx,
        chain,
        event.sender,
        event.receiver,
        event.owner,
        event.assets,
        event.shares,
        created_at,
    )
    .await?;

    db_tx.commit().await?;

    Ok(())
}
