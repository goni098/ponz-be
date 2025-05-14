use alloy::{primitives::U256, providers::Provider, sol};
use alloy_chains::NamedChain;
use shared::AppResult;

use crate::{DynChain, client::PublicClient};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    ChainLinkDataFeed,
    "src/abis/chain-link-data-feed.abi.json"
);

pub type ChainLinkDataFeedContract = ChainLinkDataFeed::ChainLinkDataFeedInstance<PublicClient>;

// 1 Unit = 10e8;
pub async fn connvert_eth_to_usd<P: Provider>(
    chain: NamedChain,
    eth_in_wei: U256,
    client: P,
) -> AppResult<U256> {
    let chain_link_data_feed_contract =
        ChainLinkDataFeed::new(chain.chain_link_data_feed_address(), client);

    let eth_price_unit: U256 = chain_link_data_feed_contract
        .latestAnswer()
        .call()
        .await?
        .try_into()
        .unwrap();

    let amount_price_unit = eth_in_wei.checked_div(U256::from(10e8)).unwrap();

    Ok(amount_price_unit.checked_mul(eth_price_unit).unwrap())
}
