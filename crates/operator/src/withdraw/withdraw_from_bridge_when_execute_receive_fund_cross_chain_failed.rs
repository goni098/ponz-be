use alloy::{primitives::U256, providers::Provider};
use alloy_chains::NamedChain;
use shared::AppResult;
use web3::{
    DynChain,
    client::WalletClient,
    contracts::{
        chain_link_datafeed::connvert_eth_to_usd,
        stargate_bridge::StargateBridge::{self, ExecuteReceiveFundCrossChainFailed},
    },
};

pub async fn withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed(
    chain: NamedChain,
    wallet_client: &WalletClient,
    event: ExecuteReceiveFundCrossChainFailed,
) -> AppResult<()> {
    let stargate_contract_address = chain.stargate_bridge_address();
    let stargate_contract = StargateBridge::new(stargate_contract_address, wallet_client);

    let tx_to_et = stargate_contract
        .executeTransferFundFromRouterToFundVaultCrossChain(
            event.guid,
            event.srcId.to::<u32>(),
            event.composeMsg.clone(),
            event.depositor,
            event.depositedTokenAddress,
            event.amount,
            U256::ZERO,
        )
        .into_transaction_request();

    let gas = wallet_client.estimate_gas(tx_to_et).await? as u128;
    let gas_price = wallet_client.get_gas_price().await?;

    let transfer_fee =
        connvert_eth_to_usd(chain, &wallet_client, U256::from(gas * gas_price)).await?;

    let pending_tx = stargate_contract
        .executeTransferFundFromRouterToFundVaultCrossChain(
            event.guid,
            event.srcId.to::<u32>(),
            event.composeMsg,
            event.depositor,
            event.depositedTokenAddress,
            event.amount,
            transfer_fee,
        )
        .send()
        .await?;

    let tx_hash = *pending_tx.tx_hash();

    tracing::info!(
        "Waiting for executeTransferFundFromRouterToFundVaultCrossChain transaction... {}",
        tx_hash
    );

    pending_tx.watch().await?;

    tracing::info!(
        "Execute executeTransferFundFromRouterToFundVaultCrossChain transaction successfully {}",
        tx_hash
    );

    Ok(())
}
