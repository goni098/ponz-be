use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, sea_query::OnConflict,
};
use web3::contracts::router::Router::DepositFund;

use crate::{entities::deposit_txn, utils::to_decimal};

pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: DepositFund,
) -> Result<(), DbErr> {
    let DepositFund {
        receiver,
        tokenAddress,
        depositAmount,
        actualDepositAmount,
        depositedAt,
    } = event;

    let created_at = DateTime::from_timestamp(depositedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid depositedAt timestamp".into()))?
        .into();

    let txn = deposit_txn::ActiveModel {
        actual_deposit_amount: Set(to_decimal(actualDepositAmount)?),
        chain_id: Set(chain as i64),
        created_at: Set(created_at),
        deposit_amount: Set(to_decimal(depositAmount)?),
        id: Default::default(),
        is_distributed: Set(false),
        receiver: Set(receiver.to_string()),
        token_address: Set(tokenAddress.to_string()),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
    };

    deposit_txn::Entity::insert(txn)
        .on_conflict(
            OnConflict::columns([deposit_txn::Column::TxHash, deposit_txn::Column::LogIndex])
                .update_columns([
                    deposit_txn::Column::Receiver,
                    deposit_txn::Column::TokenAddress,
                    deposit_txn::Column::DepositAmount,
                    deposit_txn::Column::ActualDepositAmount,
                    deposit_txn::Column::CreatedAt,
                ])
                .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}

pub async fn find_all_undistributed(
    db: &DatabaseConnection,
) -> Result<Vec<deposit_txn::Model>, DbErr> {
    deposit_txn::Entity::find()
        .filter(deposit_txn::Column::IsDistributed.eq(false))
        .all(db)
        .await
}
