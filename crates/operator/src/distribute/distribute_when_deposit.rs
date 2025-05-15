use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::contracts::router::Router::DepositFund;

pub async fn distribute_when_deposit(
    chain: NamedChain,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
    event: DepositFund,
) -> AppResult<()> {
    super::distribute(chain, db, pools_service, event.receiver, event.tokenAddress).await
}
