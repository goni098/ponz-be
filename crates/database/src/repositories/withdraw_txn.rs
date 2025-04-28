use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, prelude::DateTimeWithTimeZone,
};

use crate::{entities::withdraw_txn, utils::to_decimal};

#[allow(clippy::too_many_arguments)]
pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    receiver: Address,
    user: Address,
    strategy_address: Address,
    token_address: Address,
    share: U256,
    created_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = withdraw_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        created_at: Set(created_at),
        receiver: Set(receiver.to_string()),
        share: Set(to_decimal(share)?),
        strategy_address: Set(strategy_address.to_string()),
        token_address: Set(token_address.to_string()),
        user: Set(user.to_string()),
    };

    withdraw_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
