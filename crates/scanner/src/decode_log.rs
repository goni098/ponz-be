use alloy::{rpc::types::Log, sol_types::SolEvent};
use shared::AppResult;
use web3::contracts::{
    cross_chain_router::CrossChainRouter::TransferFundCrossChain,
    lz_executor::LzExecutor::{
        DistributeFundCrossChain, TransferFundFromRouterToFundVaultCrossChain,
    },
    referral::Refferal::Claim,
    router::Router::{
        DepositFund, DistributeUserFund, RebalanceFundSameChain, WithDrawFundSameChain,
        WithdrawRequest,
    },
    stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed,
};

use crate::ExpectedLog;

pub fn decode_log(log: Log) -> AppResult<Option<ExpectedLog>> {
    let log = match log.topic0() {
        Some(&TransferFundCrossChain::SIGNATURE_HASH) => {
            Some(ExpectedLog::TransferFundCrossChain(decode(log)?))
        }
        Some(&DistributeFundCrossChain::SIGNATURE_HASH) => {
            Some(ExpectedLog::DistributeFundCrossChain(decode(log)?))
        }
        Some(&Claim::SIGNATURE_HASH) => Some(ExpectedLog::Claim(decode(log)?)),
        Some(&DepositFund::SIGNATURE_HASH) => Some(ExpectedLog::DepositFund(decode(log)?)),
        Some(&DistributeUserFund::SIGNATURE_HASH) => {
            Some(ExpectedLog::DistributeUserFund(decode(log)?))
        }
        Some(&RebalanceFundSameChain::SIGNATURE_HASH) => {
            Some(ExpectedLog::RebalanceFundSameChain(decode(log)?))
        }
        Some(&WithDrawFundSameChain::SIGNATURE_HASH) => {
            Some(ExpectedLog::WithDrawFundSameChain(decode(log)?))
        }
        Some(&TransferFundFromRouterToFundVaultCrossChain::SIGNATURE_HASH) => Some(
            ExpectedLog::TransferFundFromRouterToFundVaultCrossChain(decode(log)?),
        ),
        Some(&WithdrawRequest::SIGNATURE_HASH) => Some(ExpectedLog::WithdrawRequest(decode(log)?)),
        Some(&ExecuteReceiveFundCrossChainFailed::SIGNATURE_HASH) => Some(
            ExpectedLog::ExecuteReceiveFundCrossChainFailed(decode(log)?),
        ),
        _ => None,
    };

    Ok(log)
}

fn decode<T>(log: Log) -> AppResult<Log<T>>
where
    T: alloy::sol_types::SolEvent,
{
    let log = log.log_decode::<T>()?;
    Ok(log)
}
