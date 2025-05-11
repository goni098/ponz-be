use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;

use crate::client::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Strategy,
    "src/abis/strategy.abi.json"
);

pub type StrategyContract = Strategy::StrategyInstance<PublicClient>;

impl StrategyContract {
    pub fn address_by_chain(_chain: NamedChain) -> Address {
        Address::ZERO
    }
}
