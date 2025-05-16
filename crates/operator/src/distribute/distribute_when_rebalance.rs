use alloy_chains::NamedChain;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::contracts::router::Router::RebalanceFundSameChain;

use super::components::distribute;

pub async fn distribute_when_rebalance(
    chain: NamedChain,
    pools_service: &ExternalPoolsService,
    event: RebalanceFundSameChain,
) -> AppResult<()> {
    distribute(
        chain,
        pools_service,
        event.userAddress,
        event.underlyingAsset,
    )
    .await
}
