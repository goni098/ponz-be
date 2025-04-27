use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "withdraw_txn")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub chain_id: i64,
    pub sender: String,
    pub receiver: String,
    pub owner: String,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub assets: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub shares: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
