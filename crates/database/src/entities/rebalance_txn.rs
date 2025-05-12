use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "rebalance_txn")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub chain_id: i64,
    pub strategy_address: String,
    pub user_address: String,
    pub underlying_asset: String,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub received_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub received_reward: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub protocol_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub rebalance_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub referral_fee: Decimal,
    pub log_index: i64,
    pub tx_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
