use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Strategy,
    "src/abis/strategy.abi.json"
);

pub fn address(chain: NamedChain) -> Address {
    match chain {
        NamedChain::Base => Address::ZERO,
        _ => Address::ZERO,
    }
}
