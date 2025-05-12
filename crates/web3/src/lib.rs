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
use alloy::{network::EthereumWallet, primitives::Address, providers::ProviderBuilder};
use alloy_chains::NamedChain;
use client::{PublicClient, WalletClient};
use serde_json::Value;
use shared::env::ENV;
use url::Url;

pub mod addresses;
pub mod client;
pub mod contracts;

pub enum Strategy {
    Balancer,
    AllBridge,
    Aerodrome,
}

pub trait EventArgs {
    fn json_args(&self) -> Value;
}

pub trait DynChain {
    fn public_client(&self) -> PublicClient;
    fn wallet_client(&self, wallet: EthereumWallet) -> WalletClient;
    fn rpc_url(&self) -> Url;
    fn router_contract_address(&self) -> Address;
    fn cross_chain_router_contract_address(&self) -> Address;
    fn fund_vault_contract_address(&self) -> Address;
    fn refferal_contract_address(&self) -> Address;
}

impl DynChain for NamedChain {
    fn public_client(&self) -> PublicClient {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .with_chain(*self)
            .connect_http(self.rpc_url())
    }

    fn wallet_client(&self, wallet: EthereumWallet) -> WalletClient {
        ProviderBuilder::new()
            .wallet(wallet)
            .with_chain(*self)
            .connect_http(self.rpc_url())
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
}
