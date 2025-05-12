use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::router::Router::RebalanceFundSameChain;

use crate::{entities::rebalance_txn, utils::to_decimal};

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

    let created_at = DateTime::from_timestamp(rebalancedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid rebalancedAt timestamp".into()))?
        .into();

    let txn = rebalance_txn::ActiveModel {
        chain_id: Set(chain as i64),
        created_at: Set(created_at),
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
    };

    rebalance_txn::Entity::insert(txn)
        .on_conflict(
            OnConflict::columns([
                rebalance_txn::Column::TxHash,
                rebalance_txn::Column::LogIndex,
            ])
            .update_columns([
                rebalance_txn::Column::StrategyAddress,
                rebalance_txn::Column::UserAddress,
                rebalance_txn::Column::UnderlyingAsset,
                rebalance_txn::Column::ReceivedAmount,
                rebalance_txn::Column::ReceivedReward,
                rebalance_txn::Column::ProtocolFee,
                rebalance_txn::Column::ReferralFee,
                rebalance_txn::Column::RebalanceFee,
                rebalance_txn::Column::CreatedAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
