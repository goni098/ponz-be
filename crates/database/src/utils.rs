use std::str::FromStr;

use sea_orm::{DbErr, prelude::Decimal};

pub fn to_decimal<T: ToString>(val: T) -> Result<Decimal, DbErr> {
    Decimal::from_str(&val.to_string()).map_err(|error| DbErr::Custom(error.to_string()))
}
