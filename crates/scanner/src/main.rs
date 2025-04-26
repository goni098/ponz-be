// use alloy::{
//     network::EthereumWallet,
//     primitives::{Address, U256},
//     providers::Provider,
// };
// use web3::{
//     client::{public_client, wallet_client},
//     contracts::router::{RouterCommonType::DepositParam, RouterContract},
// };

use std::time::Duration;

use alloy::{
    primitives::Address,
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption, FilterSet},
    sol_types::SolEventInterface,
};
use redis::AsyncCommands;
use shared::{AppResult, env::ENV};
use tokio::time::sleep;
use web3::{
    client::{CLIENTS, PublicClient},
    contracts::{fund_vault::FundVault, router::Router},
};

#[tokio::main]
async fn main() -> AppResult<()> {
    // let wallet_client = wallet_client("rpc_url".parse().unwrap(), EthereumWallet::default());
    // let client = public_client("rpc_url".parse().unwrap());

    // let contract = RouterContract::new(Address::ZERO, client);

    let client = redis::Client::open(ENV.redis_url.as_str())?;

    let mut conn = client.get_multiplexed_async_connection().await?;

    loop {
        let _: () = conn.publish("event_contract_channel", "cmm").await?;
        sleep(Duration::from_secs(6)).await;
    }
}

async fn scan(client: &PublicClient) -> AppResult<()> {
    let filter = Filter {
        address: FilterSet::from(vec![Address::ZERO, Address::ZERO]),
        block_option: FilterBlockOption::Range {
            from_block: (),
            to_block: (),
        },
        ..Default::default()
    };

    let logs = client.get_logs(&filter).await?;

    let events = logs.into_iter().map(|log| {
        if log.address() == Address::ZERO {
            FundVault::FundVaultEvents::decode_log(&log.inner)
        } else {
            Router::RouterEvents::decode_log(&log.inner)
        }
    });

    Ok(())
}
