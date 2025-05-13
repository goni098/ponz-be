use std::collections::HashMap;

use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use shared::AppResult;
use web3::contracts::{
    cross_chain_router::RouterCommonType::WithdrawStrategySameChain,
    router::Router::WithdrawRequest, strategy::Strategy,
};

#[derive(Default)]
pub struct TokenAsset {
    pub total_amount: U256,
    pub un_distributed_withdraw_amount: U256,
    pub native_value: U256,
    pub withdraw_strategy_same_chains: Vec<WithdrawStrategySameChain>,
}

pub async fn merge_tokens_from_withdraw_request<P: Provider>(
    client: P,
    event: &WithdrawRequest,
) -> AppResult<HashMap<Address, TokenAsset>> {
    let mut strategy_contract = Strategy::new(Address::ZERO, client);

    let mut tokens: HashMap<Address, TokenAsset> = HashMap::new();

    for asset_in_vault in &event.unDistributedWithdraw {
        let asset = tokens.entry(asset_in_vault.tokenAddress).or_default();

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

        let asset = tokens.entry(token_address).or_default();

        asset.total_amount += token_amount;
        asset
            .withdraw_strategy_same_chains
            .push(WithdrawStrategySameChain {
                externalCallData: asset_in_strategy.externalCallData.clone(),
                share: asset_in_strategy.share,
                strategyAddress: asset_in_strategy.strategyAddress,
            });
    }

    Ok(tokens)
}
