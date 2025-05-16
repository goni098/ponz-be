use std::collections::HashMap;

use database::models::SupportedPool;
use reqwest::Client;
use serde::Deserialize;
use shared::{AppResult, util::to_chain};

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
            supported_pool.name == token.name && supported_pool.address == token.pool_address
        }) {
            let apr = token.apr.parse()?;
            let apr_7d = token.apr7d.parse()?;
            let apr_30d = token.apr30d.parse()?;
            let tvl = token.pool_info.total_lp_amount.parse()?;

            let pool_address = pool.address.parse()?;
            let token_address = pool.token_address.parse()?;
            let strategy_address = pool.strategy_address.parse()?;

            pools_info.push(ExternalPoolInfo {
                apr,
                apr_30d,
                tvl,
                apr_7d,
                name: pool.name.clone(),
                platform: pool.platform.clone(),
                pool_address,
                token_address,
                chain: to_chain(pool.chain_id as u64)?,
                strategy_address,
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
                    token_address: "0xF3F2b4815A58152c9BE53250275e8211163268BA".to_string(),
                    chain_id: 8453,
                    enable: true,
                    id: 1,
                    platform: "noname".to_string(),
                    strategy_address: "0x5ADB96e1728Eb6493C2E0033eC70F829CaD83b1b".to_string(),
                    apr_list: Some(vec![]),
                },
                SupportedPool {
                    name: "USD Coin".to_string(),
                    address: "0x690e66fc0F8be8964d40e55EdE6aEBdfcB8A21Df".to_string(),
                    token_address: "0xF3F2b4815A58152c9BE53250275e8211163268BA".to_string(),
                    chain_id: 8453,
                    enable: true,
                    id: 1,
                    platform: "noname".to_string(),
                    strategy_address: "0xc61f0F0a752e0c04f6eFb0f0465A0b74a54185C4".to_string(),
                    apr_list: Some(vec![]),
                },
            ],
        )
        .await?;

        println!("pools: {:#?}", pools);

        Ok(())
    }
}
