use alloy::sol;

use crate::client::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    CrossChainRouter,
    "src/abis/cross-chain-router.abi.json"
);

pub type CrossChainRouterContract = CrossChainRouter::CrossChainRouterInstance<PublicClient>;
