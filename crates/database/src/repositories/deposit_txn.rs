use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, prelude::DateTimeWithTimeZone,
};

use crate::{entities::deposit_txn, utils::to_decimal};

pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    receiver: Address,
    token_address: Address,
    deposit_amount: U256,
    actual_deposit_amount: U256,
    created_at: DateTimeWithTimeZone,
    deposited_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = deposit_txn::ActiveModel {
        actual_deposit_amount: Set(to_decimal(actual_deposit_amount)?),
        chain_id: Set(chain as i64),
        created_at: Set(created_at),
        deposit_amount: Set(to_decimal(deposit_amount)?),
        deposited_at: Set(deposited_at),
        id: Default::default(),
        is_distributed: Set(false),
        receiver: Set(receiver.to_string()),
        token_address: Set(token_address.to_string()),
    };

    deposit_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
