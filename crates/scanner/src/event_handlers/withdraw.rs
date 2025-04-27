use alloy_chains::NamedChain;
use shared::AppResult;
use web3::contracts::strategy::Strategy::Withdraw;

pub async fn handle_withdraw_event(chain: NamedChain, event: Withdraw) -> AppResult<()> {
    Ok(())
}
