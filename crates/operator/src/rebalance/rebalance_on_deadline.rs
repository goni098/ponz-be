use alloy::{primitives::ruint::aliases::U256, providers::Provider};
use alloy_chains::NamedChain;
use database::models;
use shared::AppResult;
use web3::{
    DynChain,
    client::get_wallet_client,
    contracts::{
        chain_link_datafeed::connvert_eth_to_usd,
        router::{Router, RouterCommonType::RebalanceStrategySameChain},
    },
};

pub async fn rebalance_on_deadline(
    chain: NamedChain,
    snapshot: models::DistributeSanpshot,
) -> AppResult<()> {
    let wallet_client = get_wallet_client(chain).await;
    let router_contract_address = chain.router_contract_address();
    let router_contract = Router::new(router_contract_address, wallet_client);
    let strategy_address = snapshot.strategy_address.parse()?;
    let user_address = snapshot.depositor.parse()?;

    let tx_to_et = router_contract
        .rebalanceFundSameChain(RebalanceStrategySameChain {
            isReferral: false,
            rebalancesFee: U256::ZERO,
            strategyAddress: strategy_address,
            userAddress: user_address,
        })
        .into_transaction_request();

    let gas = wallet_client.estimate_gas(tx_to_et).await? as u128;
    let gas_price = wallet_client.get_gas_price().await?;

    let rebalance_fee = connvert_eth_to_usd(chain, U256::from(gas * gas_price)).await?;

    let pending_tx = router_contract
        .rebalanceFundSameChain(RebalanceStrategySameChain {
            isReferral: false,
            rebalancesFee: rebalance_fee,
            strategyAddress: strategy_address,
            userAddress: user_address,
        })
        .send()
        .await?;

    let tx_hash = *pending_tx.tx_hash();

    tracing::info!(
        "Waiting for rebalanceFundSameChain transaction... {}",
        tx_hash
    );

    pending_tx.watch().await?;

    tracing::info!(
        "Execute rebalanceFundSameChain transaction successfully {}",
        tx_hash
    );

    Ok(())
}
