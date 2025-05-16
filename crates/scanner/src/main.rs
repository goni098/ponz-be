use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption},
};
use alloy_chains::NamedChain;
use database::{
    repositories::{self, setting::Setting},
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use futures_util::future::try_join_all;
use scanner::handlers::{Context, save_log};
use shared::{AppResult, env::ENV};
use tokio::time::sleep;
use web3::{
    DynChain,
    clients::{PublicClient, create_public_client},
    events::EXPECTED_EVENTS,
    logs::decoder::decode_log,
};

#[tokio::main]
async fn main() {
    shared::logging::set_up(["scanner"]);
    let chain = shared::arg::parse_chain_arg();
    bootstrap(chain).await.unwrap();
}

const MAX_RANGE: u64 = 499;

async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    let client = create_public_client(chain);

    let router_address = chain.router_contract_address();
    let cross_chain_router_address = chain.cross_chain_router_contract_address();
    let referral_address = chain.refferal_contract_address();
    let lz_executor_address = chain.lz_executor_address();
    let stargate_bridge_address = chain.stargate_bridge_address();

    let current_scanned_block = {
        let scanned_block = repositories::setting::find(&db, Setting::ScannedBlock(chain)).await?;

        if let Some(scanned_block) = scanned_block {
            scanned_block.parse()?
        } else {
            client.get_block_number().await?
        }
    };

    let mut filter = Filter::new()
        .address(vec![
            router_address,
            cross_chain_router_address,
            referral_address,
            lz_executor_address,
            stargate_bridge_address,
        ])
        .events(EXPECTED_EVENTS)
        .from_block(BlockNumberOrTag::Number(current_scanned_block));

    tracing::info!("router_address: {}", router_address);
    tracing::info!("cross_chain_router_address: {}", cross_chain_router_address);
    tracing::info!("referral_address: {}", referral_address);
    tracing::info!("lz_executor_address: {}", lz_executor_address);
    tracing::info!("stargate_bridge_address: {}", stargate_bridge_address);

    tracing::info!("🦀 starting scanner on {}...", chain);

    loop {
        match scan(chain, &client, &db, &mut filter).await {
            Ok(next) => {
                filter = filter.from_block(next);
            }
            Err(error) => {
                tracing::error!(
                    "scan from {} to {} failed {:#?}",
                    filter.get_from_block().unwrap_or_default(),
                    filter.get_to_block().unwrap_or_default(),
                    error
                );
            }
        };

        sleep(Duration::from_secs(6)).await;
    }
}

async fn scan(
    chain: NamedChain,
    client: &PublicClient,
    db: &DatabaseConnection,
    filter: &mut Filter,
) -> AppResult<BlockNumberOrTag> {
    let latest_block = client.get_block_number().await?;
    let from_block = filter.get_from_block().unwrap_or(latest_block);
    let to_block = latest_block.min(from_block + MAX_RANGE);

    filter.block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumberOrTag::Number(from_block)),
        to_block: Some(BlockNumberOrTag::Number(to_block)),
    };

    let logs = client.get_logs(filter).await?;

    let mut tasks = Vec::with_capacity(logs.len());

    for log in logs {
        if let Some(log) = decode_log(log)? {
            tasks.push(save_log(db, chain, log, Context::Scanner));
        };
    }

    let needed_logs = tasks.len();

    try_join_all(tasks).await?;

    repositories::setting::set(db, Setting::ScannedBlock(chain), to_block.to_string()).await?;

    tracing::info!(
        "scanned from {} to {} successfully with {} logs",
        from_block,
        to_block,
        needed_logs
    );

    Ok(BlockNumberOrTag::Number(to_block))
}
