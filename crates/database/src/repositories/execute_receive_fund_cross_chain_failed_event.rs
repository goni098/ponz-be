use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed;

use crate::{
    entities::execute_receive_fund_cross_chain_failed_event, enums::TxnStatus, utils::to_decimal,
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
