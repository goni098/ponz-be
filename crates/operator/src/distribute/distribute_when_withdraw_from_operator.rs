use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::{
    client::WalletClient,
    contracts::cross_chain_router::CrossChainRouter::WithdrawFundCrossChainFromOperator,
};

pub async fn distribute_when_withdraw_from_operator(
    chain: NamedChain,
    wallet_client: &WalletClient,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
    event: WithdrawFundCrossChainFromOperator,
) -> AppResult<()> {
    super::distribute(
        chain,
        wallet_client,
        db,
        pools_service,
        event.receiver,
        event.tokenOut,
    )
    .await
}
