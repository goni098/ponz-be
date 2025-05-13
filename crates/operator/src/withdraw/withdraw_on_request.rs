use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use alloy_chains::NamedChain;
use shared::{AppError, AppResult};
use web3::{
    DynChain,
    client::wallet_client,
    contracts::{
        chain_link_datafeed::connvert_eth_to_usd,
        cross_chain_router::{
            CrossChainRouter,
            RouterCommonType::{
                SwapParam, WithdrawSameChainFromOperator, WithdrawStrategyMultipleChainsV2,
                WithdrawStrategySameChainUndistributed,
            },
        },
        router::Router::WithdrawRequest,
        stargate_bridge::StargateBridge,
    },
};

use crate::{bridges::stargate, withdraw::merge_asset::merge_tokens_from_withdraw_request};

pub async fn withdraw_on_request(dst_chain: NamedChain, event: WithdrawRequest) -> AppResult<()> {
    let source_chain: NamedChain = event
        .chainId
        .to::<u64>()
        .try_into()
        .map_err(|_| AppError::Custom("Invalid chain id from WithdrawRequest event".into()))?;

    if source_chain == dst_chain {
        withdraw_same_chain(source_chain, event).await
    } else {
        withdraw_cross_chain(source_chain, dst_chain, event).await
    }
}

async fn withdraw_same_chain(chain: NamedChain, event: WithdrawRequest) -> AppResult<()> {
    let wallet = EthereumWallet::default();
    let wallet_client = wallet_client(chain, wallet);

    let cross_router_contract =
        CrossChainRouter::new(chain.cross_chain_router_contract_address(), &wallet_client);

    let tokens = merge_tokens_from_withdraw_request(&wallet_client, &event).await?;

    let withdraw_same_chain_from_operators: Vec<WithdrawSameChainFromOperator> = tokens
        .into_iter()
        .map(|(token_address, asset)| WithdrawSameChainFromOperator {
            swapParam: SwapParam {
                amountOutMin: U256::ZERO,
                isV3: false,
                externalCallData: Bytes::new(),
                swapImpl: Address::ZERO,
                tokenReceive: Address::ZERO,
            },
            tokenIn: token_address,
            withdrawStrategySameChains: asset.withdraw_strategy_same_chains,
            unDistributedWithdraw: WithdrawStrategySameChainUndistributed {
                tokenAddress: token_address,
                unDistributedAmount: asset.un_distributed_withdraw_amount,
            },
        })
        .collect();

    let tx_to_et = cross_router_contract
        .withdrawFundSameChain(
            withdraw_same_chain_from_operators.clone(),
            event.user,
            false,
            U256::ZERO,
        )
        .into_transaction_request();

    let gas = wallet_client.estimate_gas(tx_to_et).await?;

    let withdraw_fee = connvert_eth_to_usd(chain, U256::from(gas), &wallet_client).await?;

    let pending_tx = cross_router_contract
        .withdrawFundSameChain(
            withdraw_same_chain_from_operators,
            event.user,
            false,
            withdraw_fee,
        )
        .send()
        .await?;

    let tx_hash = pending_tx.tx_hash();

    tracing::info!(
        "Waiting for withdrawFundSameChain transaction... {}",
        tx_hash
    );

    pending_tx.watch().await?;

    Ok(())
}

async fn withdraw_cross_chain(
    source_chain: NamedChain,
    dst_chain: NamedChain,
    event: WithdrawRequest,
) -> AppResult<()> {
    let wallet = EthereumWallet::default();
    let wallet_client = wallet_client(source_chain, wallet);
    let cross_router_contract = CrossChainRouter::new(
        source_chain.cross_chain_router_contract_address(),
        &wallet_client,
    );
    let stargate_bridge_contract = StargateBridge::new(
        source_chain.cross_chain_router_contract_address(),
        &wallet_client,
    );

    let tokens = stargate::estimate_withdraw(dst_chain, &wallet_client, &event).await?;

    let transport_msg = stargate_bridge_contract
        .prepareTransportMsg(dst_chain as u32, 0)
        .call()
        .await?;

    let total_value_to_send = tokens
        .iter()
        .fold(U256::ZERO, |value, (_, asset)| asset.native_value + value);

    let withdraw_strategy_multiple_chains_v2: Vec<WithdrawStrategyMultipleChainsV2> = tokens
        .into_iter()
        .map(|(token_address, asset)| WithdrawStrategyMultipleChainsV2 {
            crossChain: source_chain.stargate_bridge_address(),
            nativeValue: asset.native_value,
            transportMsg: transport_msg.clone(),
            slippage: U256::from(50),
            swapParam: SwapParam {
                amountOutMin: U256::ZERO,
                externalCallData: Bytes::new(),
                isV3: false,
                swapImpl: Address::ZERO,
                tokenReceive: Address::ZERO,
            },
            tokenIn: token_address,
            unDistributedWithdraw: WithdrawStrategySameChainUndistributed {
                tokenAddress: token_address,
                unDistributedAmount: asset.un_distributed_withdraw_amount,
            },
            withdrawStrategySameChains: asset.withdraw_strategy_same_chains,
        })
        .collect();

    let tx_to_et = cross_router_contract
        .withdrawFundAnotherChain(
            withdraw_strategy_multiple_chains_v2.clone(),
            event.user,
            false,
            U256::ZERO,
        )
        .value(total_value_to_send)
        .into_transaction_request();

    let gas = wallet_client.estimate_gas(tx_to_et).await?;

    let withdraw_fee = connvert_eth_to_usd(
        source_chain,
        U256::from(U256::from(gas) + total_value_to_send),
        &wallet_client,
    )
    .await?;

    let pending_tx = cross_router_contract
        .withdrawFundAnotherChain(
            withdraw_strategy_multiple_chains_v2,
            event.user,
            false,
            withdraw_fee,
        )
        .value(total_value_to_send)
        .send()
        .await?;

    let tx_hash = pending_tx.tx_hash();

    tracing::info!(
        "Waiting for withdrawFundAnotherChain transaction... {}",
        tx_hash
    );

    pending_tx.watch().await?;

    Ok(())
}
