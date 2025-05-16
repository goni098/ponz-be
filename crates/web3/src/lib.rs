use addresses::{arb_sepolia, arbitrum, base, sepolia};
use alloy::primitives::Address;
use alloy_chains::NamedChain;
use shared::env::ENV;
use url::Url;

pub mod addresses;
pub mod clients;
pub mod contracts;
pub mod events;
pub mod logs;

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
    fn supported_chains() -> [NamedChain; 4];
}

impl DynChain for NamedChain {
    fn ws_url(&self) -> &Url {
        match self {
            NamedChain::Base => &ENV.base_ws_url,
            NamedChain::Arbitrum => &ENV.arbitrum_ws_url,
            NamedChain::Sepolia => &ENV.sepolia_ws_url,
            NamedChain::ArbitrumSepolia => &ENV.arbitrum_ws_url,
            _ => panic!("can not resolve rpc_url, unsupported chain {}", self),
        }
    }

    fn rpc_url(&self) -> Url {
        match self {
            NamedChain::Base => ENV.base_rpc_url.clone(),
            NamedChain::Arbitrum => ENV.arbitrum_rpc_url.clone(),
            NamedChain::Sepolia => ENV.sepolia_rpc_url.clone(),
            NamedChain::ArbitrumSepolia => ENV.arbitrum_rpc_url.clone(),
            _ => panic!("can not resolve rpc_url, unsupported chain {}", self),
        }
    }

    fn cross_chain_router_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => base::CROSS_ROUTER,
            NamedChain::Arbitrum => arbitrum::CROSS_ROUTER,
            NamedChain::Sepolia => sepolia::CROSS_ROUTER,
            NamedChain::ArbitrumSepolia => arb_sepolia::CROSS_ROUTER,
            _ => panic!(
                "can not resolve cross_chain_router_contract_address, unsupported chain {}",
                self
            ),
        }
    }

    fn fund_vault_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => base::FUND_VAULT,
            NamedChain::Arbitrum => arbitrum::FUND_VAULT,
            NamedChain::Sepolia => sepolia::FUND_VAULT,
            NamedChain::ArbitrumSepolia => arb_sepolia::FUND_VAULT,
            _ => panic!(
                "can not resolve fund_vault_contract_address, unsupported chain {}",
                self
            ),
        }
    }

    fn refferal_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => base::REFERRAL,
            NamedChain::Arbitrum => arbitrum::REFERRAL,
            NamedChain::Sepolia => sepolia::REFERRAL,
            NamedChain::ArbitrumSepolia => arb_sepolia::REFERRAL,
            _ => panic!(
                "can not resolve refferal_contract_address, unsupported chain {}",
                self
            ),
        }
    }

    fn router_contract_address(&self) -> Address {
        match self {
            NamedChain::Base => base::ROUTER,
            NamedChain::Arbitrum => arbitrum::ROUTER,
            NamedChain::Sepolia => sepolia::ROUTER,
            NamedChain::ArbitrumSepolia => arb_sepolia::ROUTER,
            _ => panic!(
                "can not resolve router_contract_address, unsupported chain {}",
                self
            ),
        }
    }

    fn lz_executor_address(&self) -> Address {
        match self {
            NamedChain::Base => base::LZ_EXECUTOR,
            NamedChain::Arbitrum => arbitrum::LZ_EXECUTOR,
            NamedChain::Sepolia => sepolia::LZ_EXECUTOR,
            NamedChain::ArbitrumSepolia => arb_sepolia::LZ_EXECUTOR,
            _ => panic!(
                "can not resolve lz_executor_address, unsupported chain {}",
                self
            ),
        }
    }

    fn stargate_bridge_address(&self) -> Address {
        match self {
            NamedChain::Base => base::STARGATE_BRIGDE,
            NamedChain::Arbitrum => arbitrum::STARGATE_BRIGDE,
            NamedChain::Sepolia => sepolia::STARGATE_BRIGDE,
            NamedChain::ArbitrumSepolia => arb_sepolia::STARGATE_BRIGDE,
            _ => panic!(
                "can not resolve router_contract_address, unsupported chain {}",
                self
            ),
        }
    }

    fn chain_link_data_feed_address(&self) -> Address {
        match self {
            NamedChain::Base => base::CHAIN_LINK,
            NamedChain::Arbitrum => arbitrum::CHAIN_LINK,
            NamedChain::Sepolia => sepolia::CHAIN_LINK,
            NamedChain::ArbitrumSepolia => arb_sepolia::CHAIN_LINK,
            _ => panic!(
                "can not resolve chain_link_data_feed_address, unsupported chain {}",
                self
            ),
        }
    }

    fn supported_chains() -> [NamedChain; 4] {
        [
            NamedChain::Base,
            NamedChain::Arbitrum,
            NamedChain::Sepolia,
            NamedChain::ArbitrumSepolia,
        ]
    }
}
