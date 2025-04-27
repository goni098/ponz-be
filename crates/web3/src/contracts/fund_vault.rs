use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;

use crate::client::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    FundVault,
    "src/abis/fund-vault.abi.json"
);

pub type FundVaultContract = FundVault::FundVaultInstance<PublicClient>;

pub fn address(chain: NamedChain) -> Address {
    match chain {
        NamedChain::Base => Address::ZERO,
        _ => Address::ZERO,
    }
}
