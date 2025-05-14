use alloy::sol;

use crate::client::PublicClient;

sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    StargateBridge,
    "src/abis/stargate-bridge.abi.json"
);

pub type StargateBridgeContract = StargateBridge::StargateBridgeInstance<PublicClient>;
