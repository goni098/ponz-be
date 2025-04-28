use alloy::primitives::{Address, address};
use alloy_chains::NamedChain;

pub mod client;
pub mod contracts;

// Sepolia chain
const SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS: Address =
    address!("B1B2B16Dd8dF256EA3D5Ec818b28b4A694a9f7Eb");

const SEPOLIA_ROUTER_CONTRACT_ADDRESS: Address =
    address!("087f3F9914356CD0650D62AcfaC8F15361a72827");

const SEPOLIA_ALL_BRIGDE_CONTRACT_ADDRESS: Address =
    address!("A7e1B55c85f5cfec2C6d1fb7995eea3ef848a01b");

const SEPOLIA_AERODROME_CONTRACT_ADDRESS: Address =
    address!("FF1319EA10797e04e31361779140ac7EB0449DDF");

const SEPOLIA_BALANCER_CONTRACT_ADDRESS: Address =
    address!("C7BD65B79389a8Ef52E3a7aFDA663fe11f78b361");

const SEPOLIA_REFERRAL_CONTRACT_ADDRESS: Address =
    address!("39a18374B0357572E23c17F3057a2B0D8ed684A5");

// Base chain
const BASE_CHAIN_FUND_VAULT_CONTRACT_ADDRESS: Address =
    address!("1d9aafd68c9f8b5f6deb25c8243e86c31ea9f102");

const BASE_CHAIN_ROUTER_CONTRACT_ADDRESS: Address =
    address!("1a842a4f6c9fada6230581cafbe6619d4b3aba7d");

const BASE_CHAIN_ALL_BRIGDE_CONTRACT_ADDRESS: Address =
    address!("6df81526f93cd5c66b2b509baeb91bdb832c9a85");

const BASE_CHAIN_AERODROME_CONTRACT_ADDRESS: Address =
    address!("85afFE800e3D5098Cf9aED4749E765A4a137293D");

const BASE_CHAIN_BALANCER_CONTRACT_ADDRESS: Address =
    address!("d9eC31EFcDB4d98e6578eCB70b970eC60a064Fc2");

const BASE_CHAIN_REFERRAL_CONTRACT_ADDRESS: Address =
    address!("39a18374B0357572E23c17F3057a2B0D8ed684A5");

pub fn get_fund_vault_contract_address(chain: NamedChain) -> Address {
    match chain {
        NamedChain::Base => BASE_CHAIN_FUND_VAULT_CONTRACT_ADDRESS,
        NamedChain::Sepolia => SEPOLIA_FUND_VAULT_CONTRACT_ADDRESS,
        _ => panic!("unsupported chain {}", chain),
    }
}

pub fn get_router_contract_address(chain: NamedChain) -> Address {
    match chain {
        NamedChain::Base => BASE_CHAIN_ROUTER_CONTRACT_ADDRESS,
        NamedChain::Sepolia => SEPOLIA_ROUTER_CONTRACT_ADDRESS,
        _ => panic!("unsupported chain {}", chain),
    }
}

pub fn get_referral_address(chain: NamedChain) -> Address {
    match chain {
        NamedChain::Base => BASE_CHAIN_REFERRAL_CONTRACT_ADDRESS,
        NamedChain::Sepolia => SEPOLIA_REFERRAL_CONTRACT_ADDRESS,
        _ => panic!("unsupported chain {}", chain),
    }
}

pub fn get_all_supported_stratgies(chain: NamedChain) -> [Address; 3] {
    match chain {
        NamedChain::Base => [
            SEPOLIA_ALL_BRIGDE_CONTRACT_ADDRESS,
            SEPOLIA_AERODROME_CONTRACT_ADDRESS,
            SEPOLIA_BALANCER_CONTRACT_ADDRESS,
        ],
        NamedChain::Sepolia => [
            BASE_CHAIN_ALL_BRIGDE_CONTRACT_ADDRESS,
            BASE_CHAIN_AERODROME_CONTRACT_ADDRESS,
            BASE_CHAIN_BALANCER_CONTRACT_ADDRESS,
        ],
        _ => panic!("unsupported chain {}", chain),
    }
}
