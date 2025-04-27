use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;

use crate::client::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Router,
    "src/abis/router.abi.json"
);

pub type RouterContract = Router::RouterInstance<PublicClient>;

pub fn address(chain: NamedChain) -> Address {
    match chain {
        NamedChain::Base => Address::ZERO,
        _ => Address::ZERO,
    }
}
