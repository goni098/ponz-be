use alloy_chains::NamedChain;
use shared::AppResult;
use web3::contracts::router::Router::WithDrawFundSameChain;

pub async fn handle_withdraw_event(
    chain: NamedChain,
    event: WithDrawFundSameChain,
) -> AppResult<()> {
    Ok(())
}
