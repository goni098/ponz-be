use std::str::FromStr;

use alloy::primitives::{Address, U256};
use shared::AppError;
use web3::contracts::router::Router::{DepositFund, WithdrawRequest};

use crate::{
    entities::{deposit_fund_event, pool, user, withdraw_request_event},
    utils::to_unit,
};

pub type User = user::Model;
pub type SupportedPool = pool::Model;

impl TryFrom<withdraw_request_event::Model> for WithdrawRequest {
    type Error = AppError;
    fn try_from(model: withdraw_request_event::Model) -> Result<Self, Self::Error> {
        let event = serde_json::from_value(model.args)?;
        Ok(event)
    }
}

impl TryFrom<deposit_fund_event::Model> for DepositFund {
    type Error = AppError;
    fn try_from(model: deposit_fund_event::Model) -> Result<Self, Self::Error> {
        let event = Self {
            actualDepositAmount: to_unit(model.actual_deposit_amount)?,
            depositAmount: to_unit(model.deposit_amount)?,
            depositedAt: U256::from(model.emit_at.timestamp()),
            receiver: Address::from_str(&model.receiver)?,
            tokenAddress: Address::from_str(&model.token_address)?,
        };

        Ok(event)
    }
}
