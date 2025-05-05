use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::enums::ContractEventName;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "contract_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub chain_id: i64,
    pub contract_address: String,
    pub tx_hash: String,
    pub log_index: i32,
    pub name: ContractEventName,
    #[sea_orm(column_type = "JsonBinary")]
    pub args: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
