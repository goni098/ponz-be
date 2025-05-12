use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::{DateTime, Days, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, sea_query::OnConflict,
};
use web3::contracts::router::Router::DistributeUserFund;

use crate::{entities::distribute_txn, utils::to_decimal};

pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: DistributeUserFund,
) -> Result<(), DbErr> {
    let DistributeUserFund {
        strategyAddress,
        depositor,
        depositedTokenAddress,
        amount,
        swapContract,
        underlyingAsset,
        actualAmountOut,
        strategyShare,
        distributedFee,
        distributedAt,
    } = event;

    let created_at = DateTime::from_timestamp(distributedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid distributedAt timestamp".into()))?
        .into();

    let txn = distribute_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        amount: Set(to_decimal(amount)?),
        created_at: Set(created_at),
        deposited_token_address: Set(depositedTokenAddress.to_string()),
        depositor: Set(depositor.to_string()),
        strategy_address: Set(strategyAddress.to_string()),
        swap_contract: Set(swapContract.to_string()),
        actual_amount_out: Set(to_decimal(actualAmountOut)?),
        distributed_fee: Set(to_decimal(distributedFee)?),
        log_index: Set(log_index as i64),
        strategy_share: Set(to_decimal(strategyShare)?),
        tx_hash: Set(tx_hash.to_string()),
        underlying_asset: Set(underlyingAsset.to_string()),
        is_rebalanced: Set(false),
    };

    distribute_txn::Entity::insert(txn)
        .on_conflict(
            OnConflict::columns([
                distribute_txn::Column::TxHash,
                distribute_txn::Column::LogIndex,
            ])
            .update_columns([
                distribute_txn::Column::StrategyAddress,
                distribute_txn::Column::Depositor,
                distribute_txn::Column::DepositedTokenAddress,
                distribute_txn::Column::Amount,
                distribute_txn::Column::SwapContract,
                distribute_txn::Column::UnderlyingAsset,
                distribute_txn::Column::ActualAmountOut,
                distribute_txn::Column::StrategyShare,
                distribute_txn::Column::DistributedFee,
                distribute_txn::Column::CreatedAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}

pub async fn find_all_unrebalanced_and_order_than_10days(
    db: &DatabaseConnection,
) -> Result<Vec<distribute_txn::Model>, DbErr> {
    let date = Utc::now()
        .checked_sub_days(Days::new(10))
        .ok_or(DbErr::Custom("sub days error".to_string()))?;

    distribute_txn::Entity::find()
        .filter(distribute_txn::Column::IsRebalanced.eq(false))
        .filter(distribute_txn::Column::CreatedAt.lte(date))
        .all(db)
        .await
}
