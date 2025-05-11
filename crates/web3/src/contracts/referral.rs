use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;

use crate::{
    addresses::{base::BASE_REFERRAL_CONTRACT_ADDRESS, sepolia::SEPOLIA_REFERRAL_CONTRACT_ADDRESS},
    client::PublicClient,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Refferal,
    "src/abis/referral.abi.json"
);

pub type RefferalContract = Refferal::RefferalInstance<PublicClient>;

impl RefferalContract {
    pub fn address_by_chain(chain: NamedChain) -> Address {
        match chain {
            NamedChain::Base => BASE_REFERRAL_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_REFERRAL_CONTRACT_ADDRESS,
            _ => panic!("RefferalContract unsupported chain {}", chain),
        }
    }
}
