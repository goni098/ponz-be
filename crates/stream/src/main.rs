use alloy::{
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use alloy_chains::NamedChain;
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use futures_util::StreamExt;
use scanner::{
    decode_log::{self, ContractLog},
    log_handlers::{Context, save_log},
};
use shared::{AppResult, env::ENV};
use web3::{
    DynChain,
    client::ws_client,
    contracts::{
        cross_chain_router::CrossChainRouter::TransferFundCrossChain,
        lz_executor::LzExecutor::{
            DistributeFundCrossChain, TransferFundFromRouterToFundVaultCrossChain,
        },
        referral::Refferal::Claim,
        router::Router::{
            DepositFund, DistributeUserFund, RebalanceFundSameChain, WithDrawFundSameChain,
            WithdrawRequest,
        },
        stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed,
    },
};

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
        .events([
            // cross chain router
            TransferFundCrossChain::SIGNATURE,
            // lz executor
            DistributeFundCrossChain::SIGNATURE,
            TransferFundFromRouterToFundVaultCrossChain::SIGNATURE,
            // referral
            Claim::SIGNATURE,
            // router
            DepositFund::SIGNATURE,
            DistributeUserFund::SIGNATURE,
            RebalanceFundSameChain::SIGNATURE,
            WithDrawFundSameChain::SIGNATURE,
            WithdrawRequest::SIGNATURE,
            // stargate bridge
            ExecuteReceiveFundCrossChainFailed::SIGNATURE,
        ]);

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

async fn process_log(chain: NamedChain, log: ContractLog) -> AppResult<()> {
    match log {
        ContractLog::WithdrawRequest(log) => {
            operator::withdraw::withdraw_on_request(chain, log.inner.data).await?;
        }
        _ => {}
    };

    Ok(())
}
