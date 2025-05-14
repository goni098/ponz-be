use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::{DateTime, Days, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::OnConflict,
};
use web3::contracts::router::Router::DistributeUserFund;

use crate::{entities::distribute_user_fund_event, enums::TxnStatus, utils::to_decimal};

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

    let emit_at = DateTime::from_timestamp(distributedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid DistributeUserFund distributedAt timestamp".into(),
        ))?
        .into();

    let model = distribute_user_fund_event::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        amount: Set(to_decimal(amount)?),
        emit_at: Set(emit_at),
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
        smf_error_msg: Set(None),
        rebalance_status: Set(TxnStatus::Pending),
    };

    distribute_user_fund_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                distribute_user_fund_event::Column::TxHash,
                distribute_user_fund_event::Column::LogIndex,
            ])
            .update_columns([
                distribute_user_fund_event::Column::StrategyAddress,
                distribute_user_fund_event::Column::Depositor,
                distribute_user_fund_event::Column::DepositedTokenAddress,
                distribute_user_fund_event::Column::Amount,
                distribute_user_fund_event::Column::SwapContract,
                distribute_user_fund_event::Column::UnderlyingAsset,
                distribute_user_fund_event::Column::ActualAmountOut,
                distribute_user_fund_event::Column::StrategyShare,
                distribute_user_fund_event::Column::DistributedFee,
                distribute_user_fund_event::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}

pub async fn find_all_unrebalanced_and_order_than_10days(
    db: &DatabaseConnection,
) -> Result<Vec<distribute_user_fund_event::Model>, DbErr> {
    let date = Utc::now()
        .checked_sub_days(Days::new(10))
        .ok_or(DbErr::Custom("sub days error".to_string()))?;

    distribute_user_fund_event::Entity::find()
        .filter(distribute_user_fund_event::Column::RebalanceStatus.eq(TxnStatus::Pending))
        .filter(distribute_user_fund_event::Column::EmitAt.lte(date))
        .all(db)
        .await
}

pub async fn find_unresolved(
    db: &DatabaseConnection,
    limit: u64,
) -> Result<Vec<distribute_user_fund_event::Model>, DbErr> {
    distribute_user_fund_event::Entity::find()
        .filter(
            distribute_user_fund_event::Column::RebalanceStatus
                .is_in([TxnStatus::Failed, TxnStatus::Pending]),
        )
        .limit(limit)
        .order_by_desc(distribute_user_fund_event::Column::EmitAt)
        .all(db)
        .await
}

pub async fn pin_as_resolved<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
) -> Result<(), DbErr> {
    distribute_user_fund_event::Entity::update_many()
        .filter(distribute_user_fund_event::Column::Id.eq(tx_hash.to_string()))
        .filter(distribute_user_fund_event::Column::LogIndex.eq(log_index))
        .col_expr(
            distribute_user_fund_event::Column::RebalanceStatus,
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
    distribute_user_fund_event::Entity::update_many()
        .filter(distribute_user_fund_event::Column::Id.eq(tx_hash.to_string()))
        .filter(distribute_user_fund_event::Column::LogIndex.eq(log_index))
        .col_expr(
            distribute_user_fund_event::Column::RebalanceStatus,
            Expr::value(TxnStatus::Failed),
        )
        .col_expr(
            distribute_user_fund_event::Column::SmfErrorMsg,
            Expr::value(error_msg),
        )
        .exec(db)
        .await?;

    Ok(())
}
