use database::models::SupportedPool;
use serde::Deserialize;
use serde_json::{Value, json};
use shared::{AppError, AppResult, util::to_chain};

use crate::ExternalPoolInfo;

pub async fn fectch_pool_info(
    client: &reqwest::Client,
    pool: &SupportedPool,
) -> AppResult<ExternalPoolInfo> {
    let res = client
        .post("https://api-v3.balancer.fi/graphql")
        .json(&json!({
            "query": QUERY,
            "variables": {
                "id": pool.address,
                "chain": pool.name
            }
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let dyn_data_raw = res
        .get("data")
        .and_then(|data| data.get("pool"))
        .and_then(|pool| pool.get("dynamicData"))
        .ok_or(AppError::Custom("not found dynamicData".into()))?;

    let dyn_data: DynData = serde_json::from_value(dyn_data_raw.clone())?;

    let apr = dyn_data
        .apr_items
        .into_iter()
        .fold(0f64, |total_apr, apr_item| {
            if pool.apr_list.as_ref().is_some_and(|apr_list| {
                apr_list.iter().any(|title| title.contains(&apr_item.title))
            }) {
                total_apr + apr_item.apr
            } else {
                total_apr
            }
        });

    let liquidity = dyn_data.total_liquidity.parse()?;

    let pool_info = ExternalPoolInfo {
        apr,
        apr_7d: apr,
        apr_30d: apr,
        chain: to_chain(pool.chain_id as u64)?,
        name: pool.name.clone(),
        platform: pool.platform.clone(),
        pool_address: pool.address.parse()?,
        strategy_address: pool.strategy_address.parse()?,
        token_address: pool.token_address.parse()?,
        tvl: liquidity,
    };

    Ok(pool_info)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DynData {
    apr_items: Vec<AprItem>,
    total_liquidity: String,
}

#[derive(Deserialize)]
struct AprItem {
    apr: f64,
    title: String,
}

const QUERY: &str = r#####"query GetPool($id: String!, $chain: GqlChain!, $userAddress: String) {
        pool: poolGetPool(id: $id, chain: $chain, userAddress: $userAddress) {
            id
            address
            name

            dynamicData {
            totalLiquidity
            totalShares
            fees24h
            surplus24h
            swapFee
            volume24h
            aprItems {

                title
                apr
                rewardTokenSymbol
            }
            }
            ... on GqlPoolStable {
            poolTokens {
                ...PoolTokens
                __typename
            }
            __typename
            }
            __typename
        }
    }


    fragment Hook on GqlHook {
    address
    config {
        enableHookAdjustedAmounts
        shouldCallAfterAddLiquidity
        shouldCallAfterInitialize
        shouldCallAfterRemoveLiquidity
        shouldCallAfterSwap
        shouldCallBeforeAddLiquidity
        shouldCallBeforeInitialize
        shouldCallBeforeRemoveLiquidity
        shouldCallBeforeSwap
        shouldCallComputeDynamicSwapFee
        __typename
    }
    type
    params {
        ... on ExitFeeHookParams {
        exitFeePercentage
        __typename
        }
        ... on FeeTakingHookParams {
        addLiquidityFeePercentage
        removeLiquidityFeePercentage
        swapFeePercentage
        __typename
        }
        ... on StableSurgeHookParams {
        maxSurgeFeePercentage
        surgeThresholdPercentage
        __typename
        }
        ... on MevTaxHookParams {
        mevTaxThreshold
        mevTaxMultiplier
        maxMevSwapFeePercentage
        __typename
        }
        __typename
    }
    reviewData {
        reviewFile
        summary
        warnings
        __typename
    }
    __typename
    }

    fragment PoolTokens on GqlPoolTokenDetail {
        id
        chain
        chainId
        address
        decimals
        name
        symbol
        balanceUSD
        priceRate
        priceRateProvider
        __typename
    }"#####;

#[cfg(test)]
mod test {
    use database::models::SupportedPool;
    use shared::AppResult;

    // cargo test --package pools --lib -- balancer::test::test_fecth_balancer --exact --show-output
    #[tokio::test]
    async fn test_fecth_balancer() -> AppResult<()> {
        let client = reqwest::Client::new();

        let pool_info = super::fectch_pool_info(
            &client,
            &SupportedPool {
                address: "0x7ab124ec4029316c2a42f713828ddf2a192b36db".to_string(),
                name: "BASE".to_string(),
                chain_id: 8453,
                enable: true,
                id: 1,
                platform: "balancer".to_string(),
                strategy_address: "0xCeEA7925d55b373CCDE30D993eB6d627C2704cec".to_string(),
                token_address: "0xF3F2b4815A58152c9BE53250275e8211163268BA".to_string(),
                apr_list: Some(vec![
                    "waBasGHO APR".to_string(),
                    "GHO reward APR".to_string(),
                    "waBasUSDC APR".to_string(),
                    "Swap fees APR".to_string(),
                ]),
            },
        )
        .await?;

        println!("pool_info: {:#?}", pool_info);

        Ok(())
    }
}
