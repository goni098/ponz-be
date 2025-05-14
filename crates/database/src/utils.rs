use std::str::FromStr;

use alloy::primitives::U256;
use sea_orm::{DbErr, prelude::Decimal};
use shared::{AppError, AppResult};

pub fn to_decimal<T: ToString>(val: T) -> Result<Decimal, DbErr> {
    Decimal::from_str(&val.to_string()).map_err(|error| DbErr::Custom(error.to_string()))
}

pub fn to_unit(val: Decimal) -> AppResult<U256> {
    U256::from_str(&val.to_string()).map_err(|error| {
        AppError::Custom(format!("can not convert decimal to U256 {}", error).into())
    })
}
