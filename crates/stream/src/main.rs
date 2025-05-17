use alloy::rpc::types::Filter;
use alloy_chains::NamedChain;
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use error::SocketError;
use futures_util::{SinkExt, StreamExt};
use pools::ExternalPoolsService;
use serde_json::json;
use shared::{AppResult, env::ENV};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use web3::{DynChain, events::EXPECTED_EVENTS};

mod error;
mod extract_msg;
mod process_log;

#[tokio::main]
async fn main() {
    shared::logging::set_up(["stream", "operator"]);
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

async fn stream(chain: NamedChain, db: &DatabaseConnection) -> Result<(), SocketError> {
    let ws_url = chain.ws_url();
    let pools_service = ExternalPoolsService::new(db.clone());

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

    let (stream, _) = connect_async(ws_url.as_str()).await?;

    let (mut write, mut read) = stream.split();

    let msg_subcribe = json!({
          "id": 0,
          "jsonrpc": "2.0",
          "method": "eth_subscribe",
          "params": ["logs", filter]
    });

    write
        .send(Message::Text(msg_subcribe.to_string().into()))
        .await?;

    tracing::info!("🦀 stream is running on {}", chain);

    while let Some(message) = read.next().await {
        if let Some(log) = extract_msg::extract(message?)? {
            match process_log::process(chain, db, &pools_service, log).await {
                Ok(tx_hash) => {
                    tracing::info!("catched log {}", tx_hash);
                }
                Err(error) => {
                    tracing::error!("process log error: {:#?}", error);
                }
            };
        }
    }

    Ok(())
}
