use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, TxHash},
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption, Log},
    sol_types::{SolEvent, SolEventInterface},
};
use alloy_chains::NamedChain;
use chrono::Utc;
use database::{
    repositories::{self, setting::Setting},
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use event_handlers::{Event, handler};
use futures_util::future::try_join_all;
use shared::{AppResult, env::ENV};
use tokio::time::sleep;
use web3::{
    client::{PublicClient, public_client},
    contracts::{
        referral::Refferal::{Claim, RefferalEvents},
        router::Router::{
            DepositFund, DistributeUserFund, RebalanceFundSameChain, RouterEvents,
            WithDrawFundSameChain,
        },
    },
};

mod event_handlers;

const MAX_RANGE: u64 = 10_000;

pub async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    shared::logging::set_up("scanner");
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    let client = public_client(chain);

    let router_address = web3::get_router_contract_address(chain);
    let referral_address = web3::get_referral_address(chain);

    let current_scanned_block = {
        let scanned_block = repositories::setting::find(&db, Setting::ScannedBlock(chain)).await?;

        if let Some(scanned_block) = scanned_block {
            scanned_block.parse()?
        } else {
            client.get_block_number().await?
        }
    };

    let mut filter = Filter::new()
        .address(vec![router_address, referral_address])
        .events([
            DepositFund::SIGNATURE,
            DistributeUserFund::SIGNATURE,
            RebalanceFundSameChain::SIGNATURE,
            WithDrawFundSameChain::SIGNATURE,
            Claim::SIGNATURE,
        ])
        .from_block(BlockNumberOrTag::Number(current_scanned_block));

    tracing::info!("🦀 starting scanner on {}...", chain);

    loop {
        match scan(
            chain,
            &client,
            &db,
            &mut filter,
            router_address,
            referral_address,
        )
        .await
        {
            Ok(next) => {
                tracing::info!(
                    "scanned from {} to {} successfully",
                    filter.get_from_block().unwrap_or_default(),
                    filter.get_to_block().unwrap_or_default(),
                );
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
    router_address: Address,
    referral_address: Address,
) -> AppResult<BlockNumberOrTag> {
    let latest_block = client.get_block_number().await?;
    let from_block = filter.get_from_block().unwrap_or(latest_block);
    let to_block = latest_block.min(from_block + MAX_RANGE);

    filter.block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumberOrTag::Number(from_block)),
        to_block: Some(BlockNumberOrTag::Number(to_block)),
    };

    let logs = client.get_logs(filter).await?;

    let tx_hash_and_log_index_list: Vec<(TxHash, u64)> = logs
        .iter()
        .filter_map(|log| log.transaction_hash.zip(log.log_index))
        .collect();

    let resolved_list =
        repositories::contract_event::find_existed(db, &tx_hash_and_log_index_list).await?;

    let logs: Vec<Log> = logs
        .into_iter()
        .filter(|log| {
            log.transaction_hash
                .as_ref()
                .map(ToString::to_string)
                .zip(log.log_index)
                .is_some_and(|tx_hash_and_log_index| resolved_list.contains(&tx_hash_and_log_index))
        })
        .collect();

    let mut tasks = Vec::with_capacity(logs.len());

    for log in logs {
        let block_timestamp = log
            .block_timestamp
            .unwrap_or_else(|| Utc::now().timestamp() as u64);

        let tx_hash = log.transaction_hash.expect("exclude none above");
        let log_index = log.log_index.expect("exclude above") as i32;

        let contract_address = log.address();

        if contract_address == router_address {
            let decoded_log = RouterEvents::decode_log(&log.inner)?;

            match decoded_log.data {
                RouterEvents::DepositFund(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        log_index,
                        chain,
                        Event::Deposit(event),
                        block_timestamp,
                    ));
                }
                RouterEvents::DistributeUserFund(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        log_index,
                        chain,
                        Event::Distribute(event),
                        block_timestamp,
                    ));
                }
                RouterEvents::RebalanceFundSameChain(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        log_index,
                        chain,
                        Event::Rebalance(event),
                        block_timestamp,
                    ));
                }
                RouterEvents::WithDrawFundSameChain(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        log_index,
                        chain,
                        Event::Withdraw(event),
                        block_timestamp,
                    ));
                }
                _ => {}
            }
        } else if contract_address == referral_address {
            let decoded_log = RefferalEvents::decode_log(&log.inner)?;

            if let RefferalEvents::Claim(event) = decoded_log.data {
                tasks.push(handler(
                    db,
                    contract_address,
                    tx_hash,
                    log_index,
                    chain,
                    Event::Claim(event),
                    block_timestamp,
                ));
            }
        }
    }

    try_join_all(tasks).await?;

    repositories::setting::set(db, Setting::ScannedBlock(chain), to_block.to_string()).await?;

    Ok(BlockNumberOrTag::Number(to_block))
}
