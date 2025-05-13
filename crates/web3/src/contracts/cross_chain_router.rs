use alloy::sol;
use serde_json::Value;

use crate::{EventArgs, client::PublicClient};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    CrossChainRouter,
    "src/abis/cross-chain-router.abi.json"
);

pub type CrossChainRouterContract = CrossChainRouter::CrossChainRouterInstance<PublicClient>;

impl EventArgs for CrossChainRouter::TransferFundCrossChain {
    fn json_args(&self) -> Value {
        serde_json::json!({
            "actualAmountBridgeCrossChain": self.actualAmountBridgeCrossChain.to_string(),
            "crossChainDexReceiver": self.crossChainDexReceiver.to_string(),
            "crossChainDexSender": self.crossChainDexSender.to_string(),
            "depositedToken": self.depositedToken.to_string(),
            "depositor": self.depositor.to_string(),
            "distributionFee": self.distributionFee.to_string(),
            "strategyDestination": self.strategyDestination.to_string(),
            "tokenInForBridge": self.tokenInForBridge.to_string(),
            "tokenOutForBridge": self.tokenOutForBridge.to_string(),
            "transferFundCrossChainAt": self.transferFundCrossChainAt.to_string()
        })
    }
}
