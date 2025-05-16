use alloy::sol;

use crate::clients::PublicClient;

sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    #[derive(Debug)]
    LzExecutor,
    "src/abis/lz-executor.abi.json"
);

pub type LzExecutorContract = LzExecutor::LzExecutorInstance<PublicClient>;
