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
        fund_vault::{self, FundVault},
        router::{
            self,
            Router::{self, RouterEvents},
        },
        strategy::{self, Strategy},
    },
};

mod event_handlers;

pub async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    let client = public_client(chain);

    let router_address = router::address(chain);
    let fund_vault_address = fund_vault::address(chain);
    let strategy_address = strategy::address(chain);

    let current_scanned_block = {
        let scanned_block = repositories::setting::find(&db, Setting::ScannedBlock(chain)).await?;

        if let Some(scanned_block) = scanned_block {
            scanned_block.parse()?
        } else {
            client.get_block_number().await?
        }
    };

    let mut filter = Filter {
        address: FilterSet::from(vec![router_address, fund_vault_address, strategy_address]),
        block_option: FilterBlockOption::Range {
            from_block: Some(BlockNumberOrTag::Number(current_scanned_block)),
            to_block: None,
        },
        ..Default::default()
    };

    loop {
        match scan(
            chain,
            &client,
            &db,
            &mut filter,
            router_address,
            fund_vault_address,
            strategy_address,
        )
        .await
        {
            Ok(next) => {
                filter = filter.from_block(next);
            }
            Err(error) => {
                tracing::error!(
                    "scan from {:?} to {:?} failed {:#?}",
                    filter.get_from_block(),
                    filter.get_to_block(),
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
    strategy_address: Address,
) -> AppResult<BlockNumberOrTag> {
    let latest_block = client.get_block_number().await?;
    let from_block = filter.get_from_block().unwrap_or(latest_block);
    let to_block = latest_block.min(from_block + 5000);

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
            let decoded_log = Router::RouterEvents::decode_log(&log.inner)?;

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
                RouterEvents::WithDrawFundSameChain(event) => {
                    tasks.push(handler(
                        db,
                        contract_address,
                        tx_hash,
                        chain,
                        Event::Withdraw(event),
                    ));
                }
                _ => {}
            }
        } else if contract_address == fund_vault_address {
            let _decoded_log = FundVault::FundVaultEvents::decode_log(&log.inner)?;
        } else if contract_address == strategy_address {
            let _decoded_log = Strategy::StrategyEvents::decode_log(&log.inner)?;
        }
    }

    try_join_all(tasks).await?;

    repositories::setting::set(db, Setting::ScannedBlock(chain), to_block.to_string()).await?;

    Ok(BlockNumberOrTag::Number(to_block))
}
