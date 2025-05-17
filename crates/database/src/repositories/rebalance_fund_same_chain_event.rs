use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{
    ActiveEnum, ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::OnConflict,
};
use web3::contracts::router::Router::RebalanceFundSameChain;

use crate::{
    MAX_RETRY_COUNT_FILTER, entities::rebalance_fund_same_chain_event, enums::TxnStatus,
    utils::to_decimal,
};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: RebalanceFundSameChain,
) -> Result<(), DbErr> {
    let RebalanceFundSameChain {
        strategyAddress,
        userAddress,
        underlyingAsset,
        receivedAmount,
        receivedReward,
        protocolFee,
        referralFee,
        rebalanceFee,
        rebalancedAt,
    } = event;

    let emit_at = DateTime::from_timestamp(rebalancedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid RebalanceFundSameChain rebalancedAt timestamp".into(),
        ))?
        .into();

    let model = rebalance_fund_same_chain_event::ActiveModel {
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        id: Default::default(),
        received_amount: Set(to_decimal(receivedAmount)?),
        received_reward: Set(to_decimal(receivedReward)?),
        strategy_address: Set(strategyAddress.to_string()),
        underlying_asset: Set(underlyingAsset.to_string()),
        user_address: Set(userAddress.to_string()),
        log_index: Set(log_index as i64),
        protocol_fee: Set(to_decimal(protocolFee)?),
        rebalance_fee: Set(to_decimal(rebalanceFee)?),
        referral_fee: Set(to_decimal(referralFee)?),
        tx_hash: Set(tx_hash.to_string()),
        distribute_status: Set(TxnStatus::Pending),
        smf_error_msg: Set(None),
        attempt_retry: Set(0),
    };

    rebalance_fund_same_chain_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                rebalance_fund_same_chain_event::Column::TxHash,
                rebalance_fund_same_chain_event::Column::LogIndex,
            ])
            .update_columns([
                rebalance_fund_same_chain_event::Column::StrategyAddress,
                rebalance_fund_same_chain_event::Column::UserAddress,
                rebalance_fund_same_chain_event::Column::UnderlyingAsset,
                rebalance_fund_same_chain_event::Column::ReceivedAmount,
                rebalance_fund_same_chain_event::Column::ReceivedReward,
                rebalance_fund_same_chain_event::Column::ProtocolFee,
                rebalance_fund_same_chain_event::Column::ReferralFee,
                rebalance_fund_same_chain_event::Column::RebalanceFee,
                rebalance_fund_same_chain_event::Column::EmitAt,
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
) -> Result<Vec<rebalance_fund_same_chain_event::Model>, DbErr> {
    rebalance_fund_same_chain_event::Entity::find()
        .filter(
            rebalance_fund_same_chain_event::Column::DistributeStatus
                .is_in([TxnStatus::Failed, TxnStatus::Pending]),
        )
        .filter(rebalance_fund_same_chain_event::Column::AttemptRetry.lt(MAX_RETRY_COUNT_FILTER))
        .limit(limit)
        .order_by_desc(rebalance_fund_same_chain_event::Column::EmitAt)
        .all(db)
        .await
}

pub async fn pin_as_resolved<T: ToString>(
    db: &DatabaseConnection,
    tx_hash: T,
    log_index: u64,
) -> Result<(), DbErr> {
    rebalance_fund_same_chain_event::Entity::update_many()
        .filter(rebalance_fund_same_chain_event::Column::TxHash.eq(tx_hash.to_string()))
        .filter(rebalance_fund_same_chain_event::Column::LogIndex.eq(log_index))
        .col_expr(
            rebalance_fund_same_chain_event::Column::DistributeStatus,
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
    rebalance_fund_same_chain_event::Entity::update_many()
        .filter(rebalance_fund_same_chain_event::Column::TxHash.eq(tx_hash.to_string()))
        .filter(rebalance_fund_same_chain_event::Column::LogIndex.eq(log_index))
        .col_expr(
            rebalance_fund_same_chain_event::Column::DistributeStatus,
            Expr::value(TxnStatus::Failed.as_enum()),
        )
        .col_expr(
            rebalance_fund_same_chain_event::Column::SmfErrorMsg,
            Expr::value(error_msg),
        )
        .col_expr(
            rebalance_fund_same_chain_event::Column::AttemptRetry,
            Expr::column(rebalance_fund_same_chain_event::Column::AttemptRetry).add(1),
        )
        .exec(db)
        .await?;

    Ok(())
}
