use alloy_chains::NamedChain;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::contracts::cross_chain_router::CrossChainRouter::WithdrawFundCrossChainFromOperator;

use super::components::distribute;

pub async fn distribute_when_withdraw_from_operator(
    chain: NamedChain,
    pools_service: &ExternalPoolsService,
    event: WithdrawFundCrossChainFromOperator,
) -> AppResult<()> {
    distribute(chain, pools_service, event.receiver, event.tokenOut).await
}
