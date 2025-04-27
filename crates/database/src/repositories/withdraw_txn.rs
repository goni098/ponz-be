use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, prelude::DateTimeWithTimeZone,
};

use crate::{
    entities::{deposit_txn, withdraw_txn},
    utils::to_decimal,
};

pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    receiver: Address,
    user: Address,
    strategy_address: Address,
    token_address: Address,
    share: U256,
) -> Result<(), DbErr> {
    let txn = withdraw_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
    };

    deposit_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
