use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "distribute_txn")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub chain_id: i64,
    pub strategy_address: String,
    pub depositor: String,
    pub deposited_token_address: String,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub amount: Decimal,
    pub swap_contract: String,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub actual_amount_out: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub distributed_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub strategy_share: Decimal,
    pub underlying_asset: String,
    pub log_index: i64,
    pub tx_hash: String,
    pub is_rebalanced: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
