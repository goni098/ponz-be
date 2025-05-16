use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
use shared::{AppResult, util::to_chain};
use web3::{
    DynChain,
    client::get_wallet_client,
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

use crate::withdraw::components::merge_assets_from_withdraw_request;

pub async fn withdraw_when_request(
    chain: NamedChain,
    db: &DatabaseConnection,
    event: WithdrawRequest,
) -> AppResult<()> {
    let src_chain: NamedChain = to_chain(event.chainId.to::<u64>())?;

    if src_chain == chain {
        withdraw_same_chain(chain, db, event).await
    } else {
        withdraw_cross_chain(chain, src_chain, db, event).await
    }
}

async fn withdraw_same_chain(
    chain: NamedChain,
    db: &DatabaseConnection,
    event: WithdrawRequest,
) -> AppResult<()> {
    let wallet_client = get_wallet_client(chain).await;
    let cross_router_contract =
        CrossChainRouter::new(chain.cross_chain_router_contract_address(), &wallet_client);

    let assets = merge_assets_from_withdraw_request(chain, &event, false).await?;

    let is_refferal = repositories::user::is_refferal_user(db, event.user).await?;

    let withdraw_same_chain_from_operators: Vec<WithdrawSameChainFromOperator> = assets
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
            is_refferal,
            U256::ZERO,
        )
        .into_transaction_request();

    let gas = wallet_client.estimate_gas(tx_to_et).await? as u128;
    let gas_price = wallet_client.get_gas_price().await?;

    let withdraw_fee = connvert_eth_to_usd(chain, U256::from(gas * gas_price)).await?;

    let pending_tx = cross_router_contract
        .withdrawFundSameChain(
            withdraw_same_chain_from_operators,
            event.user,
            is_refferal,
            withdraw_fee,
        )
        .send()
        .await?;

    let tx_hash = *pending_tx.tx_hash();

    tracing::info!(
        "Waiting for withdrawFundSameChain transaction... {}",
        tx_hash
    );

    pending_tx.watch().await?;

    tracing::info!(
        "Execute withdrawFundSameChain transaction successfully {}",
        tx_hash
    );

    Ok(())
}

async fn withdraw_cross_chain(
    dst_chain: NamedChain,
    src_chain: NamedChain,
    db: &DatabaseConnection,
    event: WithdrawRequest,
) -> AppResult<()> {
    let src_wallet_client = get_wallet_client(src_chain).await;

    let cross_router_contract = CrossChainRouter::new(
        src_chain.cross_chain_router_contract_address(),
        &src_wallet_client,
    );

    let stargate_bridge_contract = StargateBridge::new(
        src_chain.cross_chain_router_contract_address(),
        &src_wallet_client,
    );

    let assets = merge_assets_from_withdraw_request(dst_chain, &event, false).await?;

    let transport_msg = stargate_bridge_contract
        .prepareTransportMsg(dst_chain as u32, 0)
        .call()
        .await?;

    let total_value_to_send = assets.iter().fold(U256::ZERO, |value, (_, asset)| {
        asset.cross_chain_native_value_fee + value
    });

    let is_refferal = repositories::user::is_refferal_user(db, event.user).await?;

    let withdraw_strategy_multiple_chains_v2: Vec<WithdrawStrategyMultipleChainsV2> = assets
        .into_iter()
        .map(|(token_address, asset)| WithdrawStrategyMultipleChainsV2 {
            crossChain: dst_chain.stargate_bridge_address(),
            nativeValue: asset.cross_chain_native_value_fee,
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
            is_refferal,
            U256::ZERO,
        )
        .value(total_value_to_send)
        .into_transaction_request();

    let gas = src_wallet_client.estimate_gas(tx_to_et).await? as u128;
    let gas_price = src_wallet_client.get_gas_price().await?;

    let withdraw_fee = connvert_eth_to_usd(
        dst_chain,
        U256::from(U256::from(gas * gas_price) + total_value_to_send),
    )
    .await?;

    let pending_tx = cross_router_contract
        .withdrawFundAnotherChain(
            withdraw_strategy_multiple_chains_v2,
            event.user,
            is_refferal,
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
