use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "contract_event_name"
)]
pub enum ContractEventName {
    #[sea_orm(string_value = "claim")]
    Claim,
    #[sea_orm(string_value = "deposit")]
    Deposit,
    #[sea_orm(string_value = "distribute")]
    Distribute,
    #[sea_orm(string_value = "withdraw")]
    Withdraw,
}
