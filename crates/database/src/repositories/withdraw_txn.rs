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
    sender: Address,
    receiver: Address,
    owner: Address,
    assets: U256,
    shares: U256,
    created_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = withdraw_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        assets: Set(to_decimal(assets)?),
        created_at: Set(created_at),
        owner: Set(owner.to_string()),
        receiver: Set(receiver.to_string()),
        sender: Set(sender.to_string()),
        shares: Set(to_decimal(shares)?),
    };

    withdraw_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}
