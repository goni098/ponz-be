use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, prelude::DateTimeWithTimeZone,
};

use crate::{entities::claim_txn, utils::to_decimal};

pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    amount: U256,
    from: Address,
    to: Address,
    created_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = claim_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        created_at: Set(created_at),
        amount: Set(to_decimal(amount)?),
        from: Set(from.to_string()),
        to: Set(to.to_string()),
    };

    claim_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
