use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;

use crate::{
    addresses::{
        base::BASE_FUND_VAULT_CONTRACT_ADDRESS, sepolia::SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS,
    },
    client::PublicClient,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    FundVault,
    "src/abis/fund-vault.abi.json"
);

pub type FundVaultContract = FundVault::FundVaultInstance<PublicClient>;

impl FundVaultContract {
    pub fn address_by_chain(chain: NamedChain) -> Address {
        match chain {
            NamedChain::Base => BASE_FUND_VAULT_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS,
            _ => panic!("FundVaultContract unsupported chain {}", chain),
        }
    }
}
