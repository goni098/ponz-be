use alloy::sol;
use serde_json::Value;

use crate::{EventArgs, client::PublicClient};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    LzExecutor,
    "src/abis/lz-executor.abi.json"
);

pub type LzExecutorContract = LzExecutor::LzExecutorInstance<PublicClient>;

impl EventArgs for LzExecutor::DistributeFundCrossChain {
    fn json_args(&self) -> Value {
        serde_json::json!({
            "actualAmountOut": self.actualAmountOut.to_string(),
            "amount": self.amount.to_string(),
            "depositedTokenAddress": self.depositedTokenAddress.to_string(),
            "depositor": self.depositor.to_string(),
            "distributedAt": self.distributedAt.to_string(),
            "distributedFee": self.distributedFee.to_string(),
            "strategyAddress": self.strategyAddress.to_string(),
            "strategyShare": self.strategyShare.to_string(),
            "swapContract": self.swapContract.to_string(),
            "underlyingAsset": self.underlyingAsset.to_string()
        })
    }
}
