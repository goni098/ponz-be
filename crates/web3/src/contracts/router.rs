use alloy::sol;
use serde_json::Value;

use crate::{EventArgs, client::PublicClient};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Router,
    "src/abis/router.abi.json"
);

pub type RouterContract = Router::RouterInstance<PublicClient>;

impl EventArgs for Router::DepositFund {
    fn json_args(&self) -> Value {
        serde_json::json!({
            "receiver": self.receiver.to_string(),
            "tokenAddress": self.tokenAddress.to_string(),
            "depositAmount": self.depositAmount.to_string(),
            "actualDepositAmount": self.actualDepositAmount.to_string(),
            "depositedAt": self.depositedAt.to_string(),
        })
    }
}

impl EventArgs for Router::DistributeUserFund {
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
            "swapContract": self.swapContract.to_string()
        })
    }
}

impl EventArgs for Router::RebalanceFundSameChain {
    fn json_args(&self) -> Value {
        serde_json::json!({
            "protocolFee": self.protocolFee.to_string(),
            "rebalanceFee": self.rebalanceFee.to_string(),
            "rebalancedAt": self.rebalancedAt.to_string(),
            "receivedAmount": self.receivedAmount.to_string(),
            "receivedReward": self.receivedReward.to_string(),
            "referralFee": self.referralFee.to_string(),
            "strategyAddress": self.strategyAddress.to_string(),
            "underlyingAsset": self.underlyingAsset.to_string(),
            "userAddress": self.userAddress.to_string()
        })
    }
}

impl EventArgs for Router::WithDrawFundSameChain {
    fn json_args(&self) -> Value {
        serde_json::json!({
            "protocolFee": self.actualWithdrawAmount.to_string(),
            "rebalanceFee": self.receiver.to_string(),
            "rebalancedAt": self.share.to_string(),
            "receivedAmount": self.strategyAddress.to_string(),
            "receivedReward": self.tokenAddress.to_string(),
            "referralFee": self.user.to_string(),
            "strategyAddress": self.withdrawAt.to_string()
        })
    }
}
