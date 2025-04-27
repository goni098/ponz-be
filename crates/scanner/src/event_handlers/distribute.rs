use alloy_chains::NamedChain;
use shared::AppResult;
use web3::contracts::router::Router::DistributeUserFund;

pub async fn handle_distribute_event(
    chain: NamedChain,
    event: DistributeUserFund,
) -> AppResult<()> {
    Ok(())
}
