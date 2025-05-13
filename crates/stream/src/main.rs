use alloy::{providers::Provider, rpc::types::Filter};
use alloy_chains::NamedChain;
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use futures_util::StreamExt;
use scanner::{
    EXPECTED_EVENTS, ExpectedLog,
    decode_log::{self},
    log_handlers::{Context, save_log},
};
use shared::{AppResult, env::ENV};
use web3::{DynChain, client::ws_client};

#[tokio::main]
async fn main() {
    shared::logging::set_up("stream");
    let chain = shared::arg::parse_chain_arg();
    bootstrap(chain).await.unwrap();
}

async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    shared::logging::set_up("stream");
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    loop {
        match stream(chain, &db).await {
            Ok(_) => {}
            Err(error) => {}
        }
    }
}

async fn stream(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    let ws_client = ws_client(chain).await?;

    let router_address = chain.router_contract_address();
    let cross_chain_router_address = chain.cross_chain_router_contract_address();
    let referral_address = chain.refferal_contract_address();
    let lz_executor_address = chain.lz_executor_address();
    let stargate_bridge_address = chain.stargate_bridge_address();

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

    while let Some(log) = stream.next().await {
        if let Some(log) = decode_log::decode_log(log)? {
            save_log(db, chain, log.clone(), Context::Stream).await?;
            match process_log(chain, log).await {
                Ok(_) => {}
                Err(error) => {}
            };
        }
    }

    Ok(())
}

async fn process_log(chain: NamedChain, log: ExpectedLog) -> AppResult<()> {
    match log {
        ExpectedLog::WithdrawRequest(log) => {
            operator::withdraw::withdraw_on_request(chain, log.inner.data).await?;
        }
        _ => {}
    };

    Ok(())
}
