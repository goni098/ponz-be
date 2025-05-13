use alloy::sol;
use serde_json::Value;

use crate::{EventArgs, client::PublicClient};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    StargateBridge,
    "src/abis/stargate-bridge.abi.json"
);

pub type StargateBridgeContract = StargateBridge::StargateBridgeInstance<PublicClient>;

impl EventArgs for StargateBridge::ExecuteReceiveFundCrossChainFailed {
    fn json_args(&self) -> Value {
        serde_json::json!({
            "amount": self.amount.to_string(),
            "composeMsg": self.composeMsg.to_string(),
            "depositedTokenAddress": self.depositedTokenAddress.to_string(),
            "depositor": self.depositor.to_string(),
            "executedAt": self.executedAt.to_string(),
            "guid": self.guid.to_string(),
            "srcId": self.srcId.to_string()
        })
    }
}
