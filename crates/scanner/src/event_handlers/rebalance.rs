use alloy_chains::NamedChain;
use shared::AppResult;
use web3::contracts::router::Router::RebalanceFundSameChain;

pub async fn handle_rebalance_event(
    chain: NamedChain,
    event: RebalanceFundSameChain,
) -> AppResult<()> {
    Ok(())
}
