use addresses::{
    base::{
        BASE_FUND_VAULT_CONTRACT_ADDRESS, BASE_REFERRAL_CONTRACT_ADDRESS,
        BASE_ROUTER_CONTRACT_ADDRESS,
    },
    sepolia::{
        SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS, SEPOLIA_REFERRAL_CONTRACT_ADDRESS,
        SEPOLIA_ROUTER_CONTRACT_ADDRESS,
    },
};
use alloy::primitives::Address;
use alloy_chains::NamedChain;
use shared::env::ENV;
use strum::IntoEnumIterator;
use url::Url;

pub mod addresses;
pub mod client;
pub mod contracts;

pub enum StrategyPool {
    Balancer,
    AllBridge,
    Aerodrome,
}

pub trait DynChain {
    fn rpc_url(&self) -> Url;
    fn ws_url(&self) -> &Url;
    fn router_contract_address(&self) -> Address;
    fn cross_chain_router_contract_address(&self) -> Address;
    fn fund_vault_contract_address(&self) -> Address;
    fn refferal_contract_address(&self) -> Address;
    fn stargate_bridge_address(&self) -> Address;
    fn lz_executor_address(&self) -> Address;
    fn chain_link_data_feed_address(&self) -> Address;
}

impl DynChain for NamedChain {
    fn ws_url(&self) -> &Url {
        match self {
            NamedChain::Sepolia => &ENV.sepolia_rpc_url,
            NamedChain::Base => &ENV.base_rpc_url,
            _ => panic!("unsupported chain {}", self),
        }
    }

    fn rpc_url(&self) -> Url {
        match self {
            NamedChain::Sepolia => ENV.sepolia_rpc_url.clone(),
            NamedChain::Base => ENV.base_rpc_url.clone(),
            _ => panic!("unsupported chain {}", self),
        }
    }

    fn cross_chain_router_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => BASE_FUND_VAULT_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS,
            _ => panic!("FundVaultContract unsupported chain {}", self),
        }
    }

    fn fund_vault_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => BASE_FUND_VAULT_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS,
            _ => panic!("FundVaultContract unsupported chain {}", self),
        }
    }

    fn refferal_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => BASE_REFERRAL_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_REFERRAL_CONTRACT_ADDRESS,
            _ => panic!("RefferalContract unsupported chain {}", self),
        }
    }

    fn router_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => BASE_ROUTER_CONTRACT_ADDRESS,
            NamedChain::Sepolia => SEPOLIA_ROUTER_CONTRACT_ADDRESS,
            _ => panic!("RouterContract unsupported chain {}", self),
        }
    }

    fn lz_executor_address(&self) -> Address {
        Address::ZERO
    }

    fn stargate_bridge_address(&self) -> Address {
        Address::ZERO
    }

    fn chain_link_data_feed_address(&self) -> Address {
        Address::ZERO
    }
}
