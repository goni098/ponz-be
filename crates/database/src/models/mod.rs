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
        deposit_fund_event, distribute_user_fund_event,
        execute_receive_fund_cross_chain_failed_event, pool, rebalance_fund_same_chain_event, user,
        withdraw_fund_cross_chain_from_operator_event, withdraw_request_event,
    },
    utils::{to_signed_unit, to_unit},
};

pub type User = user::Model;
pub type SupportedPool = pool::Model;
pub type DistributeSanpshot = distribute_user_fund_event::Model;

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
#[cfg(test)]
mod test {

    use sea_orm::prelude::{DateTimeWithTimeZone, Decimal};
    use serde_json::json;
    use web3::contracts::router::Router::{DepositFund, WithdrawRequest};

    use crate::{
        entities::{deposit_fund_event, withdraw_request_event},
        enums::TxnStatus,
    };

    // cargo test --package database --lib -- models::test::convert_withdraw_request_event --exact --show-output
    #[test]
    fn convert_withdraw_request_event() {
        let model = withdraw_request_event::Model {
            chain_id: 8453,
            emit_at: DateTimeWithTimeZone::default(),
            id: 1,
            log_index: 164,
            status: TxnStatus::Pending,
            smf_error_msg: None,
            token_out: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string(),
            tx_hash: "0x494f26cdf774d9d1201a1156f38920c163b9c5aefc5314351e6086d465f57006"
                .to_string(),
            args: json!({
                "user": "0xe6112c5c057da3e9a04b7c45ee73b68f32f941a9",
                "chainId": "0xa4b1",
                "tokenOut": "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
                "requestedAt": "0x68254727",
                "unDistributedWithdraw": [
                    {
                    "tokenAddress": "0xaf88d065e77c8cc2239327c5edb3a432268e5831",
                    "unDistributedAmount": "0xf43b5"
                    }
                ],
                "withdrawStrategySameChains": []
            }),
            attempt_retry: 0,
        };

        let event = WithdrawRequest::try_from(model);

        assert!(event.is_ok());

        println!("event: {:#?}", event);
    }

    // cargo test --package database --lib -- models::test::convert_deposit_fund_event --exact --show-output
    #[test]
    fn convert_deposit_fund_event() {
        let model = deposit_fund_event::Model {
            id: 1,
            chain_id: 8453,
            emit_at: DateTimeWithTimeZone::parse_from_str(
                "2025-05-13 02:42:33+0700",
                "%Y-%m-%d %H:%M:%S%z",
            )
            .unwrap(),
            log_index: 6,
            tx_hash: "0x409c68de4444f162461cefe08224a75cd16251a37129fbb55abc09abc9330088"
                .to_string(),
            actual_deposit_amount: Decimal::new(2000000, 0),
            deposit_amount: Decimal::new(2000000, 0),
            distribute_status: TxnStatus::Pending,
            smf_error_msg: None,
            receiver: "0x0D3E7FaacF6E3EaD3121Afc8f9C6f8f4245C1627".to_string(),
            token_address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string(),
            attempt_retry: 0,
        };

        let event = DepositFund::try_from(model);

        assert!(event.is_ok());

        println!("event: {:#?}", event);
    }
}
