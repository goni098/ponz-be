use alloy::primitives::{Address, I256, U256};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, prelude::DateTimeWithTimeZone,
};

use crate::{entities::rebalance_txn, utils::to_decimal};

#[allow(clippy::too_many_arguments)]
pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    strategy_address: Address,
    user_address: Address,
    underlying_asset: Address,
    received_amount: U256,
    received_reward: I256,
    created_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = rebalance_txn::ActiveModel {
        chain_id: Set(chain as i64),
        created_at: Set(created_at),
        id: Default::default(),
        received_amount: Set(to_decimal(received_amount)?),
        received_reward: Set(to_decimal(received_reward)?),
        strategy_address: Set(strategy_address.to_string()),
        underlying_asset: Set(underlying_asset.to_string()),
        user_address: Set(user_address.to_string()),
    };

    rebalance_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
