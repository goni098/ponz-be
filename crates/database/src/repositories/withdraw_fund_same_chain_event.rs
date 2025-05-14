use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::router::Router::WithDrawFundSameChain;

use crate::{entities::withdraw_fund_same_chain_event, utils::to_decimal};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: WithDrawFundSameChain,
) -> Result<(), DbErr> {
    let WithDrawFundSameChain {
        receiver,
        user,
        strategyAddress,
        tokenAddress,
        share,
        actualWithdrawAmount,
        withdrawAt,
    } = event;

    let emit_at = DateTime::from_timestamp(withdrawAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid withdrawAt timestamp".into()))?
        .into();

    let model = withdraw_fund_same_chain_event::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        emit_at: Set(emit_at),
        receiver: Set(receiver.to_string()),
        share: Set(to_decimal(share)?),
        strategy_address: Set(strategyAddress.to_string()),
        token_address: Set(tokenAddress.to_string()),
        user: Set(user.to_string()),
        actual_withdraw_amount: Set(to_decimal(actualWithdrawAmount)?),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
    };

    withdraw_fund_same_chain_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                withdraw_fund_same_chain_event::Column::TxHash,
                withdraw_fund_same_chain_event::Column::LogIndex,
            ])
            .update_columns([
                withdraw_fund_same_chain_event::Column::Receiver,
                withdraw_fund_same_chain_event::Column::User,
                withdraw_fund_same_chain_event::Column::StrategyAddress,
                withdraw_fund_same_chain_event::Column::TokenAddress,
                withdraw_fund_same_chain_event::Column::Share,
                withdraw_fund_same_chain_event::Column::ActualWithdrawAmount,
                withdraw_fund_same_chain_event::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
