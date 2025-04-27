use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, prelude::DateTimeWithTimeZone,
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

pub async fn create(
    db_tx: &DatabaseTransaction,
    name: ContractEventName,
    contract_address: Address,
    args: Value,
    chain: NamedChain,
    tx_hash: TxHash,
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
    };

    contract_event::Entity::insert(event).exec(db_tx).await?;

    Ok(())
}
