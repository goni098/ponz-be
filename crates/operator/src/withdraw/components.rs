use std::collections::HashMap;

use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use shared::{AppResult, util::to_chain};
use web3::{
    clients::get_public_client,
    contracts::{
        cross_chain_router::RouterCommonType::WithdrawStrategySameChain,
        router::Router::WithdrawRequest, strategy::Strategy,
    },
};

#[derive(Default)]
pub struct UserTokenParam {
    pub total_amount: U256,
    pub un_distributed_withdraw_amount: U256,
    pub cross_chain_native_value_fee: U256,
    pub withdraw_strategy_same_chains: Vec<WithdrawStrategySameChain>,
}

pub async fn merge_assets_from_withdraw_request(
    chain: NamedChain,
    event: &WithdrawRequest,
    estimate_cross_chain: bool,
) -> AppResult<HashMap<Address, UserTokenParam>> {
    let client = get_public_client(chain).await;
    let mut strategy_contract = Strategy::new(Address::ZERO, client);

    let mut assets: HashMap<Address, UserTokenParam> = HashMap::new();

    for asset_in_vault in &event.unDistributedWithdraw {
        let asset = assets.entry(asset_in_vault.tokenAddress).or_default();

        asset.total_amount += asset_in_vault.unDistributedAmount;
        asset.un_distributed_withdraw_amount += asset_in_vault.unDistributedAmount;
    }

    for asset_in_strategy in &event.withdrawStrategySameChains {
        strategy_contract.set_address(asset_in_strategy.strategyAddress);

        let token_amount = strategy_contract
            .convertToAssets(asset_in_strategy.share)
            .call()
            .await?;

        let token_address = strategy_contract.listUnderlyingAsset().call().await?._0;

        let asset = assets.entry(token_address).or_default();

        asset.total_amount += token_amount;
        asset
            .withdraw_strategy_same_chains
            .push(WithdrawStrategySameChain {
                externalCallData: asset_in_strategy.externalCallData.clone(),
                share: asset_in_strategy.share,
                strategyAddress: asset_in_strategy.strategyAddress,
            });
    }

    if estimate_cross_chain {
        let src_chain = to_chain(event.chainId.to::<u64>())?;

        for (token_address, asset) in assets.iter_mut() {
            let estimation = bridges::stargate::estimate_withdraw(
                src_chain,
                chain,
                event.user,
                *token_address,
                asset.total_amount,
            )
            .await?;

            asset.cross_chain_native_value_fee = estimation.valueToSend
        }
    }

    Ok(assets)
}
