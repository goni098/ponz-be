use std::str::FromStr;

use alloy::primitives::U256;
use sea_orm::{DbErr, prelude::Decimal};

pub fn to_decimal(val: U256) -> Result<Decimal, DbErr> {
    Decimal::from_str(&val.to_string()).map_err(|error| DbErr::Custom(error.to_string()))
}
