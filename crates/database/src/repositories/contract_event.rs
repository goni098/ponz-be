use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, QueryFilter, QuerySelect, prelude::DateTimeWithTimeZone,
};
use serde_json::Value;

use crate::{entities::contract_event, enums::ContractEventName};

pub async fn find_by_tx_hash(
    db: &DatabaseConnection,
    tx_hash: TxHash,
) -> Result<Option<contract_event::Model>, DbErr> {
    contract_event::Entity::find()
        .filter(contract_event::Column::TxHash.eq(tx_hash.to_string()))
        .one(db)
        .await
}

pub async fn find_existed(
    db: &DatabaseConnection,
    list: &[(TxHash, u64)],
) -> Result<Vec<(String, u64)>, DbErr> {
    let mut filter = Condition::any();

    for (tx_hash, log_index) in list {
        filter = filter
            .add(contract_event::Column::TxHash.eq(tx_hash.to_string()))
            .add(contract_event::Column::LogIndex.eq(*log_index));
    }

    contract_event::Entity::find()
        .select_only()
        .column(contract_event::Column::TxHash)
        .column(contract_event::Column::LogIndex)
        .filter(filter)
        .into_tuple()
        .all(db)
        .await
}

#[allow(clippy::too_many_arguments)]
pub async fn create(
    db_tx: &DatabaseTransaction,
    name: ContractEventName,
    contract_address: Address,
    args: Value,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: i32,
    created_at: DateTimeWithTimeZone,
) -> Result<(), DbErr> {
    let event = contract_event::ActiveModel {
        contract_address: Set(contract_address.to_string()),
        args: Set(args),
        chain_id: Set(chain as i64),
        created_at: Set(created_at),
        id: Default::default(),
        name: Set(name),
        tx_hash: Set(tx_hash.to_string()),
        log_index: Set(log_index),
    };

    contract_event::Entity::insert(event).exec(db_tx).await?;

    Ok(())
}
