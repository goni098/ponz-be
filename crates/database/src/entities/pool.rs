use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::enums::Pool;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "pool")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub platform: Pool,
    pub chain_id: i64,
    pub address: String,
    pub strategy_address: String,
    pub tvl: i32,
    pub apr: i32,
    pub swap_contract: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
