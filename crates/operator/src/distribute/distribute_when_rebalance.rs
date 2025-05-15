use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::contracts::router::Router::RebalanceFundSameChain;

pub async fn distribute_when_rebalance(
    chain: NamedChain,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
    event: RebalanceFundSameChain,
) -> AppResult<()> {
    super::distribute(
        chain,
        db,
        pools_service,
        event.userAddress,
        event.underlyingAsset,
    )
    .await
}
