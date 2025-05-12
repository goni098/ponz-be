use std::time::Duration;

use alloy::{
    consensus::BlockHeader,
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption},
    signers::k256::elliptic_curve::pkcs8::der::DateTime,
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
use shared::{AppError, AppResult, env::ENV};
use tokio::time::sleep;
use web3::{
    DynChain,
    client::PublicClient,
    contracts::{
        cross_chain_router::CrossChainRouter::{
            DistributeFundCrossChain, TransferFundCrossChain,
            TransferFundFromRouterToFundVaultCrossChain,
        },
        referral::Refferal::{Claim, RefferalEvents},
        router::Router::{
            DepositFund, DistributeUserFund, RebalanceFundSameChain, RouterEvents,
            WithDrawFundSameChain,
        },
    },
};

pub mod decode_log;
mod event_handlers;

const MAX_RANGE: u64 = 10_000;

pub async fn bootstrap(chain: NamedChain) -> AppResult<()> {
    shared::logging::set_up("scanner");
    let mut opt = ConnectOptions::new(&ENV.db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    let client = chain.public_client();

    let router_address = chain.router_contract_address();
    let referral_address = chain.refferal_contract_address();
    let cross_chain_router_address = chain.cross_chain_router_contract_address();

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
            referral_address,
            cross_chain_router_address,
        ])
        .events([
            DepositFund::SIGNATURE,
            DistributeUserFund::SIGNATURE,
            RebalanceFundSameChain::SIGNATURE,
            WithDrawFundSameChain::SIGNATURE,
            Claim::SIGNATURE,
            DistributeFundCrossChain::SIGNATURE,
            TransferFundCrossChain::SIGNATURE,
            TransferFundFromRouterToFundVaultCrossChain::SIGNATURE,
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

    let mut tasks = Vec::with_capacity(logs.len());

    for log in logs {
        let created_at = if let Some(timestamp) = log.block_timestamp {
            DateTime::from_timestamp(timestamp as i64, 0)
                .ok_or(AppError::Custom("Invalid block_timestamp".into()))?
        } else {
            let block_hash = log
                .block_hash
                .ok_or(AppError::Custom("Log missing block hash".into()))?;

            let block = client
                .get_block_by_hash(block_hash)
                .await?
                .ok_or(AppError::Custom(
                    format!("Not found block by block_hash {}", block_hash).into(),
                ))?;

            DateTime::from_timestamp(block.header.timestamp() as i64, 0)
                .ok_or(AppError::Custom("Invalid header block_timestamp".into()))?
        };

        tasks.push(handler(db, chain, event, created_at));
    }

    try_join_all(tasks).await?;

    repositories::setting::set(db, Setting::ScannedBlock(chain), to_block.to_string()).await?;

    Ok(BlockNumberOrTag::Number(to_block))
}
