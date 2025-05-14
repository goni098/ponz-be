use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::cross_chain_router::CrossChainRouter::WithdrawFundCrossChainFromOperator;

use crate::{entities::withdraw_fund_cross_chain_from_operator_event, utils::to_decimal};

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
