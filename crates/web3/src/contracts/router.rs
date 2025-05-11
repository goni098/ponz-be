use Router::DepositFund;
use alloy::{primitives::Address, sol};
use alloy_chains::NamedChain;
use serde_json::Value;

use crate::{
    addresses::{base::BASE_ROUTER_CONTRACT_ADDRESS, sepolia::SEPOLIA_ROUTER_CONTRACT_ADDRESS},
    client::PublicClient,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Router,
    "src/abis/router.abi.json"
);

pub type RouterContract = Router::RouterInstance<PublicClient>;

impl RouterContract {
    pub fn address_by_chain(chain: NamedChain) -> Address {
        match chain {
            NamedChain::Base => BASE_ROUTER_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_ROUTER_CONTRACT_ADDRESS,
            _ => panic!("RouterContract unsupported chain {}", chain),
        }
    }
}

impl DepositFund {
    pub fn as_json(&self) -> Value {
        serde_json::json!({
            "receiver": self.receiver.to_string(),
            "tokenAddress": self.tokenAddress.to_string(),
            "depositAmount": self.depositAmount.to_string(),
            "actualDepositAmount": self.actualDepositAmount.to_string(),
            "depositedAt": self.depositedAt.to_string(),
        })
    }
}
