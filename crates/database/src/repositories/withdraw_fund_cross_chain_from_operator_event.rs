use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{
    ActiveEnum, ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::OnConflict,
};
use web3::contracts::cross_chain_router::CrossChainRouter::WithdrawFundCrossChainFromOperator;

use crate::{
    entities::withdraw_fund_cross_chain_from_operator_event, enums::TxnStatus, utils::to_decimal,
};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: WithdrawFundCrossChainFromOperator,
) -> Result<(), DbErr> {
    let WithdrawFundCrossChainFromOperator {
        receiver,
        tokenOut,
        transportMsg,
        totalAmountOut,
        withdrawFee,
        withdrawAt,
    } = event;

    let emit_at = DateTime::from_timestamp(withdrawAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid WithdrawFundCrossChainFromOperator withdrawAt timestamp".into(),
        ))?
        .into();

    let model = withdraw_fund_cross_chain_from_operator_event::ActiveModel {
        id: Default::default(),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        receiver: Set(receiver.to_string()),
        token_out: Set(tokenOut.to_string()),
        total_amount_out: Set(to_decimal(totalAmountOut)?),
        transport_msg: Set(transportMsg.to_string()),
        withdraw_fee: Set(to_decimal(withdrawFee)?),
        smf_error_msg: Set(None),
        distribute_status: Set(TxnStatus::Pending),
    };

    withdraw_fund_cross_chain_from_operator_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                withdraw_fund_cross_chain_from_operator_event::Column::TxHash,
                withdraw_fund_cross_chain_from_operator_event::Column::LogIndex,
            ])
            .update_columns([
                withdraw_fund_cross_chain_from_operator_event::Column::Receiver,
                withdraw_fund_cross_chain_from_operator_event::Column::TokenOut,
                withdraw_fund_cross_chain_from_operator_event::Column::TransportMsg,
                withdraw_fund_cross_chain_from_operator_event::Column::TotalAmountOut,
                withdraw_fund_cross_chain_from_operator_event::Column::WithdrawFee,
                withdraw_fund_cross_chain_from_operator_event::Column::EmitAt,
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
) -> Result<Vec<withdraw_fund_cross_chain_from_operator_event::Model>, DbErr> {
    withdraw_fund_cross_chain_from_operator_event::Entity::find()
        .filter(
            withdraw_fund_cross_chain_from_operator_event::Column::DistributeStatus
                .is_in([TxnStatus::Failed, TxnStatus::Pending]),
        )
        .limit(limit)
        .order_by_desc(withdraw_fund_cross_chain_from_operator_event::Column::EmitAt)
        .all(db)
        .await
}

pub async fn pin_as_resolved<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
) -> Result<(), DbErr> {
    withdraw_fund_cross_chain_from_operator_event::Entity::update_many()
        .filter(
            withdraw_fund_cross_chain_from_operator_event::Column::TxHash.eq(tx_hash.to_string()),
        )
        .filter(withdraw_fund_cross_chain_from_operator_event::Column::LogIndex.eq(log_index))
        .col_expr(
            withdraw_fund_cross_chain_from_operator_event::Column::DistributeStatus,
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
    withdraw_fund_cross_chain_from_operator_event::Entity::update_many()
        .filter(
            withdraw_fund_cross_chain_from_operator_event::Column::TxHash.eq(tx_hash.to_string()),
        )
        .filter(withdraw_fund_cross_chain_from_operator_event::Column::LogIndex.eq(log_index))
        .col_expr(
            withdraw_fund_cross_chain_from_operator_event::Column::DistributeStatus,
            Expr::value(TxnStatus::Failed.as_enum()),
        )
        .col_expr(
            withdraw_fund_cross_chain_from_operator_event::Column::SmfErrorMsg,
            Expr::value(error_msg),
        )
        .exec(db)
        .await?;

    Ok(())
}
