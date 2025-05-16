use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{
    ActiveEnum, ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::OnConflict,
};
use web3::contracts::router::Router::DepositFund;

use crate::{entities::deposit_fund_event, enums::TxnStatus, utils::to_decimal};

pub async fn upsert(
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

    let emit_at = DateTime::from_timestamp(depositedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid DepositFund depositedAt timestamp".into(),
        ))?
        .into();

    let model = deposit_fund_event::ActiveModel {
        actual_deposit_amount: Set(to_decimal(actualDepositAmount)?),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        deposit_amount: Set(to_decimal(depositAmount)?),
        id: Default::default(),
        receiver: Set(receiver.to_string()),
        token_address: Set(tokenAddress.to_string()),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        smf_error_msg: Set(None),
        distribute_status: Set(TxnStatus::Pending),
    };

    deposit_fund_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                deposit_fund_event::Column::TxHash,
                deposit_fund_event::Column::LogIndex,
            ])
            .update_columns([
                deposit_fund_event::Column::Receiver,
                deposit_fund_event::Column::TokenAddress,
                deposit_fund_event::Column::DepositAmount,
                deposit_fund_event::Column::ActualDepositAmount,
                deposit_fund_event::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}

pub async fn find_unresolved(
    db: &DatabaseConnection,
    limit: u64,
) -> Result<Vec<deposit_fund_event::Model>, DbErr> {
    deposit_fund_event::Entity::find()
        .filter(
            deposit_fund_event::Column::DistributeStatus
                .is_in([TxnStatus::Failed, TxnStatus::Pending]),
        )
        .limit(limit)
        .order_by_desc(deposit_fund_event::Column::EmitAt)
        .all(db)
        .await
}

pub async fn pin_as_resolved<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
) -> Result<(), DbErr> {
    deposit_fund_event::Entity::update_many()
        .filter(deposit_fund_event::Column::TxHash.eq(tx_hash.to_string()))
        .filter(deposit_fund_event::Column::LogIndex.eq(log_index))
        .col_expr(
            deposit_fund_event::Column::DistributeStatus,
            Expr::value(TxnStatus::Done.as_enum()),
        )
        .exec(db)
        .await?;

    Ok(())
}

pub async fn pin_as_failed<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
    error_msg: String,
) -> Result<(), DbErr> {
    deposit_fund_event::Entity::update_many()
        .filter(deposit_fund_event::Column::TxHash.eq(tx_hash.to_string()))
        .filter(deposit_fund_event::Column::LogIndex.eq(log_index))
        .col_expr(
            deposit_fund_event::Column::DistributeStatus,
            Expr::value(TxnStatus::Failed.as_enum()),
        )
        .col_expr(
            deposit_fund_event::Column::SmfErrorMsg,
            Expr::value(error_msg),
        )
        .exec(db)
        .await?;

    Ok(())
}
