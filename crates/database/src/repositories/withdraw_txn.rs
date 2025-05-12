use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::router::Router::WithDrawFundSameChain;

use crate::{entities::withdraw_txn, utils::to_decimal};

#[allow(clippy::too_many_arguments)]
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

    let created_at = DateTime::from_timestamp(withdrawAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid withdrawAt timestamp".into()))?
        .into();

    let txn = withdraw_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        created_at: Set(created_at),
        receiver: Set(receiver.to_string()),
        share: Set(to_decimal(share)?),
        strategy_address: Set(strategyAddress.to_string()),
        token_address: Set(tokenAddress.to_string()),
        user: Set(user.to_string()),
        actual_withdraw_amount: Set(to_decimal(actualWithdrawAmount)?),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
    };

    withdraw_txn::Entity::insert(txn)
        .on_conflict(
            OnConflict::columns([withdraw_txn::Column::TxHash, withdraw_txn::Column::LogIndex])
                .update_columns([
                    withdraw_txn::Column::Receiver,
                    withdraw_txn::Column::User,
                    withdraw_txn::Column::StrategyAddress,
                    withdraw_txn::Column::TokenAddress,
                    withdraw_txn::Column::Share,
                    withdraw_txn::Column::ActualWithdrawAmount,
                    withdraw_txn::Column::CreatedAt,
                ])
                .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
