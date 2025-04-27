use alloy::primitives::Address;
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, prelude::DateTimeWithTimeZone,
};

use crate::entities::claim_txn;

pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    receiver: Address,
    withdrawer: Address,
    token: Address,
    created_at: DateTimeWithTimeZone,
    claimed_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = claim_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        created_at: Set(created_at),
        claimed_at: Set(claimed_at),
        token: Set(token.to_string()),
        withdrawer: Set(withdrawer.to_string()),
        receiver: Set(receiver.to_string()),
    };

    claim_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
