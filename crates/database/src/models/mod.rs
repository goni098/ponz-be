use std::str::FromStr;

use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use shared::AppError;
use web3::contracts::{
    cross_chain_router::CrossChainRouter::WithdrawFundCrossChainFromOperator,
    router::Router::{DepositFund, RebalanceFundSameChain, WithdrawRequest},
    stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed,
};

use crate::{
    entities::{
        deposit_fund_event, execute_receive_fund_cross_chain_failed_event, pool,
        rebalance_fund_same_chain_event, user, withdraw_fund_cross_chain_from_operator_event,
        withdraw_request_event,
    },
    utils::{to_signed_unit, to_unit},
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

impl TryFrom<rebalance_fund_same_chain_event::Model> for RebalanceFundSameChain {
    type Error = AppError;
    fn try_from(model: rebalance_fund_same_chain_event::Model) -> Result<Self, Self::Error> {
        let event = Self {
            protocolFee: to_unit(model.protocol_fee)?,
            rebalanceFee: to_unit(model.rebalance_fee)?,
            rebalancedAt: U256::from(model.emit_at.timestamp()),
            receivedAmount: to_unit(model.received_amount)?,
            receivedReward: to_signed_unit(model.received_reward)?,
            referralFee: to_unit(model.rebalance_fee)?,
            strategyAddress: Address::from_str(&model.strategy_address)?,
            underlyingAsset: Address::from_str(&model.underlying_asset)?,
            userAddress: Address::from_str(&model.user_address)?,
        };

        Ok(event)
    }
}

impl TryFrom<withdraw_fund_cross_chain_from_operator_event::Model>
    for WithdrawFundCrossChainFromOperator
{
    type Error = AppError;
    fn try_from(
        model: withdraw_fund_cross_chain_from_operator_event::Model,
    ) -> Result<Self, Self::Error> {
        let event = Self {
            withdrawFee: to_unit(model.withdraw_fee)?,
            totalAmountOut: to_unit(model.total_amount_out)?,
            withdrawAt: U256::from(model.emit_at.timestamp()),
            receiver: Address::from_str(&model.receiver)?,
            tokenOut: Address::from_str(&model.token_out)?,
            transportMsg: Bytes::from_str(&model.transport_msg)?,
        };

        Ok(event)
    }
}

impl TryFrom<execute_receive_fund_cross_chain_failed_event::Model>
    for ExecuteReceiveFundCrossChainFailed
{
    type Error = AppError;
    fn try_from(
        model: execute_receive_fund_cross_chain_failed_event::Model,
    ) -> Result<Self, Self::Error> {
        let event = Self {
            srcId: U256::from(model.chain_id as u64),
            amount: to_unit(model.amount)?,
            executedAt: U256::from(model.emit_at.timestamp()),
            depositedTokenAddress: Address::from_str(&model.deposited_token_address)?,
            depositor: Address::from_str(&model.depositor)?,
            composeMsg: Bytes::from_str(&model.compose_msg)?,
            guid: FixedBytes::from_str(&model.guid)?,
        };

        Ok(event)
    }
}
