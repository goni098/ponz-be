use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{
    ActiveEnum, ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::OnConflict,
};
use web3::contracts::stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed;

use crate::{
    MAX_RETRY_COUNT_FILTER, entities::execute_receive_fund_cross_chain_failed_event,
    enums::TxnStatus, utils::to_decimal,
};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: ExecuteReceiveFundCrossChainFailed,
) -> Result<(), DbErr> {
    let ExecuteReceiveFundCrossChainFailed {
        guid,
        composeMsg,
        depositor,
        depositedTokenAddress,
        amount,
        srcId,
        executedAt,
    } = event;

    let emit_at = DateTime::from_timestamp(executedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid ExecuteReceiveFundCrossChainFailed executedAt timestamp".into(),
        ))?
        .into();

    let model = execute_receive_fund_cross_chain_failed_event::ActiveModel {
        id: Default::default(),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        amount: Set(to_decimal(amount)?),
        compose_msg: Set(composeMsg.to_string()),
        deposited_token_address: Set(depositedTokenAddress.to_string()),
        depositor: Set(depositor.to_string()),
        guid: Set(guid.to_string()),
        smf_error_msg: Set(None),
        status: Set(TxnStatus::Pending),
        src_id: Set(srcId.to()),
        attempt_retry: Set(0),
    };

    execute_receive_fund_cross_chain_failed_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                execute_receive_fund_cross_chain_failed_event::Column::TxHash,
                execute_receive_fund_cross_chain_failed_event::Column::LogIndex,
            ])
            .update_columns([
                execute_receive_fund_cross_chain_failed_event::Column::Guid,
                execute_receive_fund_cross_chain_failed_event::Column::ComposeMsg,
                execute_receive_fund_cross_chain_failed_event::Column::Depositor,
                execute_receive_fund_cross_chain_failed_event::Column::DepositedTokenAddress,
                execute_receive_fund_cross_chain_failed_event::Column::Amount,
                execute_receive_fund_cross_chain_failed_event::Column::SrcId,
                execute_receive_fund_cross_chain_failed_event::Column::EmitAt,
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
) -> Result<Vec<execute_receive_fund_cross_chain_failed_event::Model>, DbErr> {
    execute_receive_fund_cross_chain_failed_event::Entity::find()
        .filter(
            execute_receive_fund_cross_chain_failed_event::Column::Status
                .is_in([TxnStatus::Failed, TxnStatus::Pending]),
        )
        .filter(
            execute_receive_fund_cross_chain_failed_event::Column::AttemptRetry
                .lt(MAX_RETRY_COUNT_FILTER),
        )
        .limit(limit)
        .order_by_desc(execute_receive_fund_cross_chain_failed_event::Column::EmitAt)
        .all(db)
        .await
}

pub async fn pin_as_resolved<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
) -> Result<(), DbErr> {
    execute_receive_fund_cross_chain_failed_event::Entity::update_many()
        .filter(
            execute_receive_fund_cross_chain_failed_event::Column::TxHash.eq(tx_hash.to_string()),
        )
        .filter(execute_receive_fund_cross_chain_failed_event::Column::LogIndex.eq(log_index))
        .col_expr(
            execute_receive_fund_cross_chain_failed_event::Column::Status,
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
    execute_receive_fund_cross_chain_failed_event::Entity::update_many()
        .filter(
            execute_receive_fund_cross_chain_failed_event::Column::TxHash.eq(tx_hash.to_string()),
        )
        .filter(execute_receive_fund_cross_chain_failed_event::Column::LogIndex.eq(log_index))
        .col_expr(
            execute_receive_fund_cross_chain_failed_event::Column::Status,
            Expr::value(TxnStatus::Failed.as_enum()),
        )
        .col_expr(
            execute_receive_fund_cross_chain_failed_event::Column::SmfErrorMsg,
            Expr::value(error_msg),
        )
        .col_expr(
            execute_receive_fund_cross_chain_failed_event::Column::AttemptRetry,
            Expr::column(execute_receive_fund_cross_chain_failed_event::Column::AttemptRetry)
                .add(1),
        )
        .exec(db)
        .await?;

    Ok(())
}
