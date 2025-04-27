use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, TxHash},
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption, FilterSet, Log},
    sol_types::SolEventInterface,
};
use alloy_chains::NamedChain;
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
        fund_vault::FundVault::FundVaultEvents, router::Router::RouterEvents,
        strategy::Strategy::StrategyEvents,
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
    let fund_vault_address = web3::get_fund_vault_contract_address(chain);
    let strategy_addresses = web3::get_all_supported_stratgies(chain);

    let current_scanned_block = {
        let scanned_block = repositories::setting::find(&db, Setting::ScannedBlock(chain)).await?;

        if let Some(scanned_block) = scanned_block {
            scanned_block.parse()?
        } else {
            client.get_block_number().await?
        }
    };

    let mut addresses_lookup = vec![router_address, fund_vault_address];
    addresses_lookup.extend(strategy_addresses);

    let mut filter = Filter {
        address: FilterSet::from(addresses_lookup),
        block_option: FilterBlockOption::Range {
            from_block: Some(BlockNumberOrTag::Number(current_scanned_block)),
            to_block: None,
        },
        ..Default::default()
    };

    tracing::info!("🦀 starting scanner on {}...", chain);

    loop {
        match scan(
            chain,
            &client,
            &db,
            &mut filter,
            router_address,
            fund_vault_address,
            &strategy_addresses,
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
    fund_vault_address: Address,
    strategy_address: &[Address],
) -> AppResult<BlockNumberOrTag> {
    let latest_block = client.get_block_number().await?;
    let from_block = filter.get_from_block().unwrap_or(latest_block);
    let to_block = latest_block.min(from_block + MAX_RANGE);

    filter.block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumberOrTag::Number(from_block)),
        to_block: Some(BlockNumberOrTag::Number(to_block)),
    };

    let logs = client.get_logs(filter).await?;

    let hashes: Vec<TxHash> = logs.iter().filter_map(|log| log.transaction_hash).collect();

    let existed_logs = repositories::contract_event::find_existed(db, &hashes).await?;

    let logs: Vec<Log> = logs
        .into_iter()
        .filter(|log| {
            log.transaction_hash
                .is_some_and(|hash| !existed_logs.contains(&hash.to_string()))
        })
        .collect();

    let mut tasks = Vec::with_capacity(logs.len());

    for log in logs {
        let tx_hash = log.transaction_hash.expect("exclude none above");
        let contract_address = log.address();

        if contract_address == router_address {
            let decoded_log = RouterEvents::decode_log(&log.inner)?;

            match decoded_log.data {
                RouterEvents::DepositFund(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        chain,
                        Event::Deposit(event),
                    ));
                }
                RouterEvents::DistributeUserFund(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        chain,
                        Event::Distribute(event),
                    ));
                }
                RouterEvents::RebalanceFundSameChain(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        chain,
                        Event::Rebalance(event),
                    ));
                }
                _ => {}
            }
        } else if contract_address == fund_vault_address {
            let _decoded_log = FundVaultEvents::decode_log(&log.inner)?;
        } else if strategy_address.contains(&contract_address) {
            let decoded_log = StrategyEvents::decode_log(&log.inner)?;

            match decoded_log.data {
                StrategyEvents::Withdraw(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        chain,
                        Event::Withdraw(event),
                    ));
                }
                StrategyEvents::ClaimRewardStrategy(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        chain,
                        Event::Claim(event),
                    ));
                }
                _ => {}
            }
        }
    }

    try_join_all(tasks).await?;

    repositories::setting::set(db, Setting::ScannedBlock(chain), to_block.to_string()).await?;

    Ok(BlockNumberOrTag::Number(to_block))
}
