use alloy_chains::NamedChain;
use shared::AppResult;
use web3::contracts::strategy::Strategy::ClaimRewardStrategy;

pub async fn handle_claim_event(chain: NamedChain, event: ClaimRewardStrategy) -> AppResult<()> {
    Ok(())
}
