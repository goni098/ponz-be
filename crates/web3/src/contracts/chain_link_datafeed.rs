use alloy::{primitives::U256, sol};
use alloy_chains::NamedChain;
use shared::{AppError, AppResult};

use crate::{
    DynChain,
    clients::{PublicClient, get_public_client},
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    ChainLinkDataFeed,
    "src/abis/chain-link-data-feed.abi.json"
);

pub type ChainLinkDataFeedContract = ChainLinkDataFeed::ChainLinkDataFeedInstance<PublicClient>;

// 1 Unit = 10e8;
pub async fn connvert_eth_to_usd(chain: NamedChain, native_wei: U256) -> AppResult<U256> {
    let client = get_public_client(chain).await;

    let chain_link_data_feed_contract =
        ChainLinkDataFeed::new(chain.chain_link_data_feed_address(), client);

    let eth_price_unit: U256 = chain_link_data_feed_contract
        .latestAnswer()
        .call()
        .await?
        .try_into()?;

    let usd_equivalent_in_wei = native_wei
        .checked_mul(eth_price_unit)
        .ok_or(AppError::Custom("Mul overflow".into()))?
        .checked_div(U256::from(10u64.pow(8)))
        .ok_or(AppError::Custom("Div to zero".into()))?;

    Ok(usd_equivalent_in_wei)
}

#[cfg(test)]
mod test {
    use alloy::primitives::utils::{format_ether, parse_ether};
    use alloy_chains::NamedChain;
    use shared::AppResult;

    // cargo test --package web3 --lib -- contracts::chain_link_datafeed::test --show-output
    #[tokio::test]
    async fn test_connvert_eth_to_usd() -> AppResult<()> {
        let _1_eth_in_wei = parse_ether("1").unwrap();
        let usd_equivalent_in_wei =
            super::connvert_eth_to_usd(NamedChain::Base, _1_eth_in_wei).await?;

        let usd_equivalent = format_ether(usd_equivalent_in_wei);

        println!("usd_equivalent {}", usd_equivalent);

        Ok(())
    }
}
