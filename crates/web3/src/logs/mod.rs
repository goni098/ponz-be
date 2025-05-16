pub mod decoder;

use alloy::rpc::types::Log;

use crate::contracts::{
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

#[derive(Clone, Debug)]
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
