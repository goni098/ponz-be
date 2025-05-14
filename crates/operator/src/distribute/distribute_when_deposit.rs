use alloy::{
    primitives::{Address, U256, ruint::support},
    providers::Provider,
};
use alloy_chains::NamedChain;
use database::{models::SupportedPool, repositories, sea_orm::DatabaseConnection};
use pools::{ExternalPoolsService, PoolParams};
use shared::{AppError, AppResult};
use web3::{
    DynChain,
    client::public_client,
    contracts::{
        fund_vault::FundVault,
        router::Router::DepositFund,
        strategy::{self, Strategy},
    },
};
struct UserStrategyInfo {
    amount: U256,
    token_address: Address,
    strategy_address: Address,
    chain: NamedChain,
}

struct UserStrategyResult {
    chain: NamedChain,
    deposit_needed_amount: U256,
    token_address: Address,
    strategy_address: Address,
}

pub async fn distribute_when_deposit(chain: NamedChain, event: DepositFund) -> AppResult<()> {
    Ok(())
}

async fn calcualte_distribute_params<P: Provider>(
    chain: NamedChain,
    user: Address,
    token_address: Address,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
) -> AppResult<()> {
    let supported_pools = repositories::pool::find_supported_pools(db).await?;

    let user_strategies = get_user_strategies(user, &supported_pools).await?;
    let best_pools = pools_service.find_all(&supported_pools).await?;

    let client = public_client(chain);
    let fund_vault_contract_address = chain.fund_vault_contract_address();
    let fund_vault_contract = FundVault::new(fund_vault_contract_address, client);

    let mut available_vault_amount = fund_vault_contract
        .getUserDepositInfor(user, token_address)
        .call()
        .await?
        .currentDepositAmount;

    for pool in best_pools {

        // let user_strategy = user_strategies.iter().find(|user_strategy| u)
    }

    Ok(())
}

async fn get_user_strategies(
    user: Address,
    supported_pools: &[SupportedPool],
) -> AppResult<Vec<UserStrategyInfo>> {
    let mut user_strategies = Vec::with_capacity(supported_pools.len());

    for pool in supported_pools {
        let chain = NamedChain::try_from(pool.chain_id as u64)
            .map_err(|_| AppError::Custom("can not convert chain from chain_id".into()))?;

        let client = public_client(chain);
        let token_address = pool.token_address.parse()?;
        let strategy_contract_address = pool.strategy_address.parse()?;
        let strategy_contract = Strategy::new(strategy_contract_address, client);

        let strategy_amount = strategy_contract
            .getActualAssets(user, token_address)
            .call()
            .await?;

        user_strategies.push(UserStrategyInfo {
            amount: strategy_amount,
            chain,
            strategy_address: strategy_contract_address,
            token_address,
        });
    }

    Ok(user_strategies)
}
