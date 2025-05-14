use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::OnConflict,
};
use serde_json::json;
use web3::contracts::router::Router::WithdrawRequest;

use crate::{entities::withdraw_request_event, enums::TxnStatus};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: WithdrawRequest,
) -> Result<(), DbErr> {
    let emit_at = DateTime::from_timestamp(event.requestedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid WithdrawRequest requestedAt timestamp".into(),
        ))?
        .into();

    let model = withdraw_request_event::ActiveModel {
        id: Default::default(),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        token_out: Set(event.tokenOut.to_string()),
        args: Set(json!(event)),
        status: Set(TxnStatus::Pending),
        smf_error_msg: Set(None),
    };

    withdraw_request_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                withdraw_request_event::Column::TxHash,
                withdraw_request_event::Column::LogIndex,
            ])
            .update_columns([
                withdraw_request_event::Column::TokenOut,
                withdraw_request_event::Column::Args,
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
) -> Result<Vec<withdraw_request_event::Model>, DbErr> {
    withdraw_request_event::Entity::find()
        .filter(
            withdraw_request_event::Column::Status.is_in([TxnStatus::Pending, TxnStatus::Failed]),
        )
        .order_by_desc(withdraw_request_event::Column::EmitAt)
        .limit(limit)
        .all(db)
        .await
}

pub async fn pin_as_resolved<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
) -> Result<(), DbErr> {
    withdraw_request_event::Entity::update_many()
        .filter(withdraw_request_event::Column::Id.eq(tx_hash.to_string()))
        .filter(withdraw_request_event::Column::LogIndex.eq(log_index))
        .col_expr(
            withdraw_request_event::Column::Status,
            Expr::value(TxnStatus::Done),
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
    withdraw_request_event::Entity::update_many()
        .filter(withdraw_request_event::Column::Id.eq(tx_hash.to_string()))
        .filter(withdraw_request_event::Column::LogIndex.eq(log_index))
        .col_expr(
            withdraw_request_event::Column::Status,
            Expr::value(TxnStatus::Failed),
        )
        .col_expr(
            withdraw_request_event::Column::SmfErrorMsg,
            Expr::value(error_msg),
        )
        .exec(db)
        .await?;

    Ok(())
}
