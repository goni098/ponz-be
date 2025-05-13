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

#[derive(Clone)]
pub enum ContractLog {
    // cross chain router
    TransferFundCrossChain(Log<TransferFundCrossChain>),
    // lz executor
    DistributeFundCrossChain(Log<DistributeFundCrossChain>),
    // referral
    Claim(Log<Claim>),
    // router
    DepositFund(Log<DepositFund>),
    DistributeUserFund(Log<DistributeUserFund>),
    RebalanceFundSameChain(Log<RebalanceFundSameChain>),
    WithDrawFundSameChain(Log<WithDrawFundSameChain>),
    WithdrawRequest(Log<WithdrawRequest>),
    TransferFundFromRouterToFundVaultCrossChain(Log<TransferFundFromRouterToFundVaultCrossChain>),
    // stargate bridge
    ExecuteReceiveFundCrossChainFailed(Log<ExecuteReceiveFundCrossChainFailed>),
}

pub fn decode_log(log: Log) -> AppResult<Option<ContractLog>> {
    let log = match log.topic0() {
        Some(&TransferFundCrossChain::SIGNATURE_HASH) => {
            Some(ContractLog::TransferFundCrossChain(decode(log)?))
        }
        Some(&DistributeFundCrossChain::SIGNATURE_HASH) => {
            Some(ContractLog::DistributeFundCrossChain(decode(log)?))
        }
        Some(&Claim::SIGNATURE_HASH) => Some(ContractLog::Claim(decode(log)?)),
        Some(&DepositFund::SIGNATURE_HASH) => Some(ContractLog::DepositFund(decode(log)?)),
        Some(&DistributeUserFund::SIGNATURE_HASH) => {
            Some(ContractLog::DistributeUserFund(decode(log)?))
        }
        Some(&RebalanceFundSameChain::SIGNATURE_HASH) => {
            Some(ContractLog::RebalanceFundSameChain(decode(log)?))
        }
        Some(&WithDrawFundSameChain::SIGNATURE_HASH) => {
            Some(ContractLog::WithDrawFundSameChain(decode(log)?))
        }
        Some(&TransferFundFromRouterToFundVaultCrossChain::SIGNATURE_HASH) => Some(
            ContractLog::TransferFundFromRouterToFundVaultCrossChain(decode(log)?),
        ),
        Some(&WithdrawRequest::SIGNATURE_HASH) => Some(ContractLog::WithdrawRequest(decode(log)?)),
        Some(&ExecuteReceiveFundCrossChainFailed::SIGNATURE_HASH) => Some(
            ContractLog::ExecuteReceiveFundCrossChainFailed(decode(log)?),
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
