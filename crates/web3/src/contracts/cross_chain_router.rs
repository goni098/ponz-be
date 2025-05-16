use alloy::sol;

use crate::clients::PublicClient;

use super::router::Router::RebalanceFundSameChain;

sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    #[derive(Debug)]
    CrossChainRouter,
    "src/abis/cross-chain-router.abi.json"
);

pub type CrossChainRouterContract = CrossChainRouter::CrossChainRouterInstance<PublicClient>;

impl From<CrossChainRouter::RebalanceFundSameChain> for RebalanceFundSameChain {
    fn from(value: CrossChainRouter::RebalanceFundSameChain) -> Self {
        Self {
            strategyAddress: value.strategyAddress,
            userAddress: value.userAddress,
            underlyingAsset: value.underlyingAsset,
            receivedAmount: value.receivedAmount,
            receivedReward: value.receivedReward,
            protocolFee: value.protocolFee,
            referralFee: value.referralFee,
            rebalanceFee: value.rebalanceFee,
            rebalancedAt: value.rebalancedAt,
        }
    }
}
