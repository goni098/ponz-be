use std::str::FromStr;

use sea_orm::{DbErr, prelude::Decimal};
use shared::{AppError, AppResult};

pub fn to_decimal<T: ToString>(val: T) -> Result<Decimal, DbErr> {
    Decimal::from_str(&val.to_string()).map_err(|error| DbErr::Custom(error.to_string()))
}

pub fn to_unit<T>(val: Decimal) -> AppResult<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    T::from_str(&val.to_string()).map_err(|error| {
        AppError::Custom(format!("can not convert decimal to unit {}", error).into())
    })
}
