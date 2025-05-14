use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::lz_executor::LzExecutor::DistributeFundCrossChain;

use crate::{entities::distribute_fund_cross_chain, utils::to_decimal};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: DistributeFundCrossChain,
) -> Result<(), DbErr> {
    let DistributeFundCrossChain {
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
            "Invalid DistributeFundCrossChain distributedAt timestamp".into(),
        ))?
        .into();

    let model = distribute_fund_cross_chain::ActiveModel {
        id: Default::default(),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        actual_amount_out: Set(to_decimal(actualAmountOut)?),
        amount: Set(to_decimal(amount)?),
        deposited_token_address: Set(depositedTokenAddress.to_string()),
        depositor: Set(depositor.to_string()),
        distributed_fee: Set(to_decimal(distributedFee)?),
        strategy_address: Set(strategyAddress.to_string()),
        strategy_share: Set(to_decimal(strategyShare)?),
        swap_contract: Set(swapContract.to_string()),
        underlying_asset: Set(underlyingAsset.to_string()),
    };

    distribute_fund_cross_chain::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                distribute_fund_cross_chain::Column::TxHash,
                distribute_fund_cross_chain::Column::LogIndex,
            ])
            .update_columns([
                distribute_fund_cross_chain::Column::StrategyAddress,
                distribute_fund_cross_chain::Column::Depositor,
                distribute_fund_cross_chain::Column::DepositedTokenAddress,
                distribute_fund_cross_chain::Column::Amount,
                distribute_fund_cross_chain::Column::SwapContract,
                distribute_fund_cross_chain::Column::UnderlyingAsset,
                distribute_fund_cross_chain::Column::ActualAmountOut,
                distribute_fund_cross_chain::Column::StrategyShare,
                distribute_fund_cross_chain::Column::DistributedFee,
                distribute_fund_cross_chain::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
