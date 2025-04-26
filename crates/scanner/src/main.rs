use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::Provider,
};
use web3::{
    client::{public_client, wallet_client},
    contracts::router::{RouterCommonType::DepositParam, RouterContract},
};

#[tokio::main]
async fn main() {
    let wallet_client = wallet_client("rpc_url".parse().unwrap(), EthereumWallet::default());
    let client = public_client("rpc_url".parse().unwrap());

    let contract = RouterContract::new(Address::ZERO, client);

    println!("Hello, world!");
}
