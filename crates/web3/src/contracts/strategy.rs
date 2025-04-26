use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Strategy,
    "src/abis/strategy.abi.json"
);
