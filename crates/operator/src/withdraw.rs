mod withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed;
mod withdraw_when_request;

use std::collections::HashMap;

use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
use shared::AppResult;
use web3::{
    client::get_public_client,
    contracts::{
        cross_chain_router::RouterCommonType::WithdrawStrategySameChain,
        router::Router::WithdrawRequest,
        stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed, strategy::Strategy,
    },
};
pub use withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed::*;
pub use withdraw_when_request::*;

pub async fn process_from_db(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    let unresolved_withdraw_req_events =
        repositories::withdraw_request_event::find_unresolved(db, 1).await?;
    let unresolved_execute_receive_fund_cross_chain_failed_events =
        repositories::execute_receive_fund_cross_chain_failed_event::find_unresolved(db, 1).await?;

    for unresolved_event in unresolved_withdraw_req_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = WithdrawRequest::try_from(unresolved_event)?;

        match withdraw_when_request(chain, event).await {
            Ok(_) => {
                repositories::withdraw_request_event::pin_as_resolved(db, tx_hash, log_index)
                    .await?;
            }
            Err(error) => {
                repositories::withdraw_request_event::pin_as_failed(
                    db,
                    tx_hash,
                    log_index,
                    format!("{:#?}", error),
                )
                .await?;
            }
        }
    }

    for unresolved_event in unresolved_execute_receive_fund_cross_chain_failed_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = ExecuteReceiveFundCrossChainFailed::try_from(unresolved_event)?;

        match withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed(chain, event).await
        {
            Ok(_) => {
                repositories::execute_receive_fund_cross_chain_failed_event::pin_as_resolved(
                    db, tx_hash, log_index,
                )
                .await?;
            }
            Err(error) => {
                repositories::execute_receive_fund_cross_chain_failed_event::pin_as_failed(
                    db,
                    tx_hash,
                    log_index,
                    format!("{:#?}", error),
                )
                .await?;
            }
        }
    }

    Ok(())
}

#[derive(Default)]
pub struct TokenAsset {
    pub total_amount: U256,
    pub un_distributed_withdraw_amount: U256,
    pub native_value: U256,
    pub withdraw_strategy_same_chains: Vec<WithdrawStrategySameChain>,
}

pub async fn merge_tokens_from_withdraw_request(
    chain: NamedChain,
    event: &WithdrawRequest,
) -> AppResult<HashMap<Address, TokenAsset>> {
    let client = get_public_client(chain).await;
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
