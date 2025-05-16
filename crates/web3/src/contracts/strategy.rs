use alloy::sol;

use crate::clients::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Strategy,
    "src/abis/strategy.abi.json"
);

pub type StrategyContract = Strategy::StrategyInstance<PublicClient>;
