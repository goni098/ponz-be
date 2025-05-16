use alloy::{
    primitives::TxHash,
    providers::Provider,
    rpc::types::{Filter, Log},
};
use alloy_chains::NamedChain;
use database::{
    repositories,
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use futures_util::StreamExt;
use operator::withdraw::withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed;
use pools::ExternalPoolsService;
use scanner::{
    EXPECTED_EVENTS, ExpectedLog,
    decode_log::{self},
    log_handlers::{Context, save_log},
};
use shared::{AppResult, env::ENV};
use web3::{DynChain, client::create_ws_client};

#[tokio::main]
async fn main() {
    shared::logging::set_up("stream");
    let chain = shared::arg::parse_chain_arg();
    bootstrap(chain).await.unwrap();
}

async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    loop {
        match stream(chain, &db).await {
            Ok(_) => {}
            Err(error) => {
                tracing::error!("websocket has disconnected: {:#?}", error);
                tracing::info!("reconnecting...");
            }
        }
    }
}

async fn stream(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    let ws_client = create_ws_client(chain).await?;

    let router_address = chain.router_contract_address();
    let cross_chain_router_address = chain.cross_chain_router_contract_address();
    let referral_address = chain.refferal_contract_address();
    let lz_executor_address = chain.lz_executor_address();
    let stargate_bridge_address = chain.stargate_bridge_address();

    tracing::info!("router_address: {}", router_address);
    tracing::info!("cross_chain_router_address: {}", cross_chain_router_address);
    tracing::info!("referral_address: {}", referral_address);
    tracing::info!("lz_executor_address: {}", lz_executor_address);
    tracing::info!("stargate_bridge_address: {}", stargate_bridge_address);

    let filter = Filter::new()
        .address(vec![
            router_address,
            cross_chain_router_address,
            referral_address,
            lz_executor_address,
            stargate_bridge_address,
        ])
        .events(EXPECTED_EVENTS);

    let mut stream = ws_client.subscribe_logs(&filter).await?.into_stream();
    let pools_service = ExternalPoolsService::new(db.clone());

    tracing::info!("🦀 stream is running on {}", chain);

    while let Some(log) = stream.next().await {
        match process_log(chain, db, &pools_service, log).await {
            Ok(tx_hash) => {
                tracing::info!("handled log {}", tx_hash);
            }
            Err(error) => {
                tracing::error!("process log error: {:#?}", error);
            }
        };
    }

    Ok(())
}

async fn process_log(
    chain: NamedChain,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
    log: Log,
) -> AppResult<TxHash> {
    let tx_hash = log.transaction_hash.unwrap_or_default();
    let log_index = log.log_index.unwrap_or_default();

    let Some(log) = decode_log::decode_log(log)? else {
        return Ok(tx_hash);
    };

    save_log(db, chain, log.clone(), Context::Stream).await?;

    match log {
        ExpectedLog::DepositFund(log) => {
            match operator::distribute::distribute_when_deposit(
                chain,
                pools_service,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::deposit_fund_event::pin_as_resolved(db, tx_hash, log_index)
                        .await?;
                }
                Err(error) => {
                    repositories::deposit_fund_event::pin_as_failed(
                        db,
                        tx_hash,
                        log_index,
                        format!("{:#?}", error),
                    )
                    .await?;
                }
            };
        }
        ExpectedLog::RebalanceFundSameChain(log) => {
            match operator::distribute::distribute_when_rebalance(
                chain,
                pools_service,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::rebalance_fund_same_chain_event::pin_as_resolved(
                        db, tx_hash, log_index,
                    )
                    .await?;
                }
                Err(error) => {
                    repositories::rebalance_fund_same_chain_event::pin_as_failed(
                        db,
                        tx_hash,
                        log_index,
                        format!("{:#?}", error),
                    )
                    .await?;
                }
            };
        }
        ExpectedLog::WithdrawFundCrossChainFromOperator(log) => {
            match operator::distribute::distribute_when_withdraw_from_operator(
                chain,
                pools_service,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::withdraw_fund_cross_chain_from_operator_event::pin_as_resolved(
                        db, tx_hash, log_index,
                    )
                    .await?;
                }
                Err(error) => {
                    repositories::withdraw_fund_cross_chain_from_operator_event::pin_as_failed(
                        db,
                        tx_hash,
                        log_index,
                        format!("{:#?}", error),
                    )
                    .await?;
                }
            };
        }
        ExpectedLog::ExecuteReceiveFundCrossChainFailed(log) => {
            match withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed(
                chain,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::execute_receive_fund_cross_chain_failed_event::pin_as_resolved(
                        db, tx_hash, log_index,
                    )
                    .await?;
                }
                Err(error) => {
                    repositories::execute_receive_fund_cross_chain_failed_event::pin_as_failed(
                        db,
                        tx_hash,
                        log_index,
                        format!("{:#?}", error),
                    )
                    .await?;
                }
            }
        }
        ExpectedLog::WithdrawRequest(log) => {
            match operator::withdraw::withdraw_when_request(chain, db, log.inner.data).await {
                Ok(_) => {
                    repositories::withdraw_request_event::pin_as_resolved(db, tx_hash, log_index)
                        .await?;
                }
                Err(error) => {
                    repositories::withdraw_request_event::pin_as_failed(
                        db,
                        tx_hash,
                        log_index,
                        format!("{:#?}", error),
                    )
                    .await?;
                }
            };
        }
        _ => {}
    };

    Ok(tx_hash)
}
