use std::collections::HashMap;

use alloy_chains::NamedChain;
use database::models::SupportedPool;
use reqwest::Client;
use serde::Deserialize;
use shared::{AppError, AppResult};

use crate::ExternalPoolInfo;

pub async fn fetch_pools_info(
    client: &Client,
    supported_pools: &[SupportedPool],
) -> AppResult<Vec<ExternalPoolInfo>> {
    let mut pools_info = vec![];

    for token in client
        .get("https://core.api.allbridgecoreapi.net/token-info?filter=all")
        .send()
        .await?
        .json::<HashMap<String, Chain>>()
        .await?
        .into_iter()
        .flat_map(|(_, chain)| chain.tokens)
    {
        if let Some(pool) = supported_pools.iter().find(|supported_pool| {
            supported_pool.name == token.name
                && supported_pool.address == token.pool_address
                && supported_pool.token_address == token.token_address
        }) {
            let apr = token.apr.parse()?;
            let apr_7d = token.apr7d.parse()?;
            let apr_30d = token.apr30d.parse()?;
            let tvl = token.pool_info.total_lp_amount.parse()?;

            pools_info.push(ExternalPoolInfo {
                apr,
                apr_30d,
                tvl,
                apr_7d,
                name: token.name,
                platform: "allbridge".to_string(),
                pool_address: token.pool_address,
                token_address: token.token_address,
                chain: NamedChain::try_from(pool.chain_id as u64)
                    .map_err(|_| AppError::Custom("can not convert chain from chain+id".into()))?,
                strategy_address: pool.strategy_address.clone(),
            });
        }
    }

    Ok(pools_info)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Token {
    name: String,
    pool_address: String,
    token_address: String,
    pool_info: PoolInfo,
    apr: String,
    apr7d: String,
    apr30d: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PoolInfo {
    total_lp_amount: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Chain {
    tokens: Vec<Token>,
}

#[cfg(test)]
mod test {
    use database::models::SupportedPool;
    use shared::AppResult;

    use crate::allbridge::fetch_pools_info;

    // cargo test --package pools --lib -- allbridge::test::test_fetching_allbridge_pools --exact --show-output
    #[tokio::test]
    async fn test_fetching_allbridge_pools() -> AppResult<()> {
        let client = reqwest::Client::new();
        let pools = fetch_pools_info(
            &client,
            &[
                SupportedPool {
                    name: "USD Coin".to_string(),
                    address: "0xDA6bb1ec3BaBA68B26bEa0508d6f81c9ec5e96d5".to_string(),
                    token_address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string(),
                    chain_id: 8453,
                    enable: true,
                    id: 1,
                    platform: "noname".to_string(),
                    strategy_address: "ignore".to_string(),
                    apr_list: vec![],
                },
                SupportedPool {
                    name: "USD Coin".to_string(),
                    address: "0x690e66fc0F8be8964d40e55EdE6aEBdfcB8A21Df".to_string(),
                    token_address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string(),
                    chain_id: 8453,
                    enable: true,
                    id: 1,
                    platform: "noname".to_string(),
                    strategy_address: "ignore".to_string(),
                    apr_list: vec![],
                },
            ],
        )
        .await?;

        dbg!(pools);

        Ok(())
    }
}
