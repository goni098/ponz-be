use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use chrono::{Days, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, prelude::DateTimeWithTimeZone,
};

use crate::{entities::distribute_txn, utils::to_decimal};

#[allow(clippy::too_many_arguments)]
pub async fn create(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    strategy_address: Address,
    depositor: Address,
    deposited_token_address: Address,
    amount: U256,
    swap_contract: Address,
    created_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let txn = distribute_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        amount: Set(to_decimal(amount)?),
        created_at: Set(created_at),
        deposited_token_address: Set(deposited_token_address.to_string()),
        depositor: Set(depositor.to_string()),
        strategy_address: Set(strategy_address.to_string()),
        swap_contract: Set(swap_contract.to_string()),
        is_rebalanced: Set(false),
    };

    distribute_txn::Entity::insert(txn).exec(db_tx).await?;

    Ok(())
}

pub async fn find_all_unrebalanced_and_order_than_10days(
    db: &DatabaseConnection,
) -> Result<Vec<distribute_txn::Model>, DbErr> {
    let date = Utc::now()
        .checked_sub_days(Days::new(10))
        .ok_or(DbErr::Custom("sub days error".to_string()))?;

    distribute_txn::Entity::find()
        .filter(distribute_txn::Column::IsRebalanced.eq(false))
        .filter(distribute_txn::Column::CreatedAt.lte(date))
        .all(db)
        .await
}
