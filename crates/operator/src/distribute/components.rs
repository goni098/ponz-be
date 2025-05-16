use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use alloy_chains::NamedChain;
use pools::{ExternalPoolInfo, ExternalPoolsService};
use shared::{AppResult, env::ENV};
use web3::{
    DynChain,
    client::{get_public_client, get_wallet_client},
    contracts::{
        chain_link_datafeed::connvert_eth_to_usd,
        fund_vault::FundVault,
        router::{
            Router,
            RouterCommonType::{DepositToStrategySameChain, SwapTokenWhenDepositParam},
        },
        strategy::Strategy,
    },
};

pub async fn distribute(
    chain: NamedChain,
    pools_service: &ExternalPoolsService,
    user: Address,
    token_address: Address,
) -> AppResult<()> {
    let strategies =
        calcualate_distribution_strategies(chain, pools_service, user, token_address).await?;

    for strategy in strategies {
        dbg!(strategy.chain);
        dbg!(chain);
        if strategy.chain == chain {
            dbg!("distribute samechain");
            distribute_same_chain(chain, user, strategy).await?;
        } else {
            distriute_cross_chain(chain, user, strategy).await?;
        }
    }

    Ok(())
}

async fn distribute_same_chain(
    chain: NamedChain,
    user: Address,
    strategy: UserStrategyResult,
) -> AppResult<()> {
    dbg!("start distribute same chain");
    let wallet_client = get_wallet_client(chain).await;
    let router_contract_address = chain.router_contract_address();
    let router_contract = Router::new(router_contract_address, wallet_client);

    dbg!(wallet_client);

    let tx_to_et = router_contract
        .depositFundToStrategySameChainFromOperator(
            DepositToStrategySameChain {
                depositedTokenAddress: strategy.token_address,
                depositor: user,
                strategyAddress: strategy.strategy_address,
                amount: strategy.deposit_needed_amount,
                distributionFee: U256::ZERO,
                externalCallData: Bytes::new(),
            },
            SwapTokenWhenDepositParam {
                amountOutMin: U256::ZERO,
                externalCallData: Bytes::new(),
                isV3: false,
                swapContract: Address::ZERO,
            },
        )
        .into_transaction_request();

    dbg!(wallet_client.get_accounts().await?);

    let gas = wallet_client.estimate_gas(tx_to_et).await? as u128;
    dbg!(gas);

    let gas_price = wallet_client.get_gas_price().await?;
    dbg!(gas_price);

    let distribution_fee = connvert_eth_to_usd(chain, U256::from(gas * gas_price)).await?;

    let pending_tx = router_contract
        .depositFundToStrategySameChainFromOperator(
            DepositToStrategySameChain {
                depositedTokenAddress: strategy.token_address,
                depositor: user,
                strategyAddress: strategy.strategy_address,
                amount: strategy.deposit_needed_amount,
                distributionFee: distribution_fee,
                externalCallData: Bytes::new(),
            },
            SwapTokenWhenDepositParam {
                amountOutMin: U256::ZERO,
                externalCallData: Bytes::new(),
                isV3: false,
                swapContract: Address::ZERO,
            },
        )
        .send()
        .await?;

    let tx_hash = *pending_tx.tx_hash();

    tracing::info!(
        "Waiting for depositFundToStrategySameChainFromOperator transaction... {}",
        tx_hash
    );

    pending_tx.watch().await?;

    tracing::info!(
        "Execute depositFundToStrategySameChainFromOperator transaction successfully {}",
        tx_hash
    );

    Ok(())
}

async fn distriute_cross_chain(
    chain: NamedChain,
    user: Address,
    strategy: UserStrategyResult,
) -> AppResult<()> {
    Ok(())
}

async fn calcualate_distribution_strategies(
    chain: NamedChain,
    pools_service: &ExternalPoolsService,
    user: Address,
    token_address: Address,
) -> AppResult<Vec<UserStrategyResult>> {
    let client = get_public_client(chain).await;

    let top_choices = pools_service.find_top_choices().await?;

    dbg!(&top_choices);
    let user_pool_infos = get_user_pool_infos_on_top_choices(user, &top_choices).await?;

    let fund_vault_contract_address = chain.fund_vault_contract_address();
    let fund_vault_contract = FundVault::new(fund_vault_contract_address, client);

    let mut available_vault_amount = fund_vault_contract
        .getUserDepositInfor(token_address, user)
        .call()
        .await?
        .currentDepositAmount;

    let distribute_target = U256::from(ENV.distribute_target);
    let distribute_min = U256::from(ENV.distribute_min);

    let mut result = vec![];

    dbg!(available_vault_amount);
    dbg!(distribute_min);

    for (target_pool, current_user_pool_info) in top_choices.iter().zip(user_pool_infos) {
        if available_vault_amount < distribute_min {
            break;
        }

        let deposit_needed_amount = U256::min(
            available_vault_amount,
            distribute_target - current_user_pool_info.amount,
        );

        result.push(UserStrategyResult {
            chain: target_pool.chain,
            deposit_needed_amount,
            strategy_address: current_user_pool_info.strategy_address,
            token_address: current_user_pool_info.strategy_address,
        });

        available_vault_amount -= deposit_needed_amount;
    }

    dbg!(&result);

    Ok(result)
}

async fn get_user_pool_infos_on_top_choices(
    user: Address,
    top_choices: &[ExternalPoolInfo],
) -> AppResult<Vec<UserPoolInfo>> {
    let mut user_pools = Vec::with_capacity(top_choices.len());

    for pool in top_choices {
        let chain = pool.chain;
        let client = get_public_client(chain).await;
        let token_address = pool.token_address.parse()?;
        let strategy_contract_address = pool.strategy_address.parse()?;
        let strategy_contract = Strategy::new(strategy_contract_address, client);

        let strategy_amount = strategy_contract
            .getActualAssets(user, token_address)
            .call()
            .await?;

        user_pools.push(UserPoolInfo {
            amount: strategy_amount,
            strategy_address: strategy_contract_address,
        });
    }

    Ok(user_pools)
}

struct UserPoolInfo {
    amount: U256,
    strategy_address: Address,
}

#[derive(Debug)]
struct UserStrategyResult {
    chain: NamedChain,
    deposit_needed_amount: U256,
    token_address: Address,
    strategy_address: Address,
}
