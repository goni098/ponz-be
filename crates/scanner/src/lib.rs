pub mod decode_log;
pub mod log_handlers;

use alloy::{rpc::types::Log, sol_types::SolEvent};
use web3::contracts::{
    cross_chain_router::CrossChainRouter::{
        RebalanceFundSameChain as RebalanceFundSameChainFromCrossRouter, TransferFundCrossChain,
        WithdrawFundCrossChainFromOperator,
    },
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
pub enum ExpectedLog {
    // cross chain router
    TransferFundCrossChain(Log<TransferFundCrossChain>),
    WithdrawFundCrossChainFromOperator(Log<WithdrawFundCrossChainFromOperator>),
    RebalanceFundSameChainFromCrossRouter(Log<RebalanceFundSameChainFromCrossRouter>),
    // lz executor
    DistributeFundCrossChain(Log<DistributeFundCrossChain>),
    ExecuteReceiveFundCrossChainFailed(Log<ExecuteReceiveFundCrossChainFailed>),
    // referral
    Claim(Log<Claim>),
    // router
    DepositFund(Log<DepositFund>),
    DistributeUserFund(Log<DistributeUserFund>),
    WithDrawFundSameChain(Log<WithDrawFundSameChain>),
    RebalanceFundSameChain(Log<RebalanceFundSameChain>),
    WithdrawRequest(Log<WithdrawRequest>),
    // stargate bridge
    TransferFundFromRouterToFundVaultCrossChain(Log<TransferFundFromRouterToFundVaultCrossChain>),
}

pub const EXPECTED_EVENTS: [&str; 12] = [
    // cross chain router
    TransferFundCrossChain::SIGNATURE,
    WithdrawFundCrossChainFromOperator::SIGNATURE,
    RebalanceFundSameChainFromCrossRouter::SIGNATURE,
    // lz executor
    DistributeFundCrossChain::SIGNATURE,
    ExecuteReceiveFundCrossChainFailed::SIGNATURE,
    // referral
    Claim::SIGNATURE,
    // router
    DepositFund::SIGNATURE,
    DistributeUserFund::SIGNATURE,
    WithDrawFundSameChain::SIGNATURE,
    RebalanceFundSameChain::SIGNATURE,
    WithdrawRequest::SIGNATURE,
    // stargate bridge
    TransferFundFromRouterToFundVaultCrossChain::SIGNATURE,
];
