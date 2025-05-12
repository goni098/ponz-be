use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "deposit_txn")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub chain_id: i64,
    pub receiver: String,
    pub token_address: String,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub deposit_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub actual_deposit_amount: Decimal,
    pub is_distributed: bool,
    pub log_index: i64,
    pub tx_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
