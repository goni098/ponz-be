use alloy::sol_types::SolEvent;

use crate::contracts::{
    cross_chain_router::CrossChainRouter::{
        //  RebalanceFundSameChain as RebalanceFundSameChainFromCrossRouter,
        TransferFundCrossChain,
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

pub const EXPECTED_EVENTS: [&str; 11] = [
    // cross chain router
    TransferFundCrossChain::SIGNATURE,
    WithdrawFundCrossChainFromOperator::SIGNATURE,
    // RebalanceFundSameChainFromCrossRouter::SIGNATURE,
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
