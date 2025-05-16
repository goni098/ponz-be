use alloy_chains::NamedChain;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::contracts::router::Router::DepositFund;

use super::components::distribute;

pub async fn distribute_when_deposit(
    chain: NamedChain,
    pools_service: &ExternalPoolsService,
    event: DepositFund,
) -> AppResult<()> {
    distribute(chain, pools_service, event.receiver, event.tokenAddress).await
}
