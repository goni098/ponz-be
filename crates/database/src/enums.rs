use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Pool {
    #[sea_orm(string_value = "Balancer")]
    Balancer,
    #[sea_orm(string_value = "AllBridge")]
    AllBridge,
    #[sea_orm(string_value = "Aerodrome")]
    Aerodrome,
    #[sea_orm(string_value = "Compound")]
    Compound,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "txn_status")]
pub enum TxnStatus {
    #[sea_orm(string_value = "done")]
    Done,
    #[sea_orm(string_value = "failed")]
    Failed,
    #[sea_orm(string_value = "pending")]
    Pending,
}
