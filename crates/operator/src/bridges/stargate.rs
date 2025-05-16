use std::collections::HashMap;

use alloy::primitives::{Address, Bytes};
use alloy_chains::NamedChain;
use shared::{AppError, AppResult, util::CheckedPercent};
use web3::{
    DynChain,
    client::get_public_client,
    contracts::{router::Router::WithdrawRequest, stargate_bridge::StargateBridge},
};

use crate::withdraw::components::{TokenAsset, merge_tokens_from_withdraw_request};

pub async fn estimate_withdraw(
    dst_chain: NamedChain,
    event: &WithdrawRequest,
) -> AppResult<HashMap<Address, TokenAsset>> {
    let src_chain: NamedChain = event
        .chainId
        .to::<u64>()
        .try_into()
        .map_err(|_| AppError::Custom("Invalid chain id from WithdrawRequest event".into()))?;

    let src_client = get_public_client(src_chain).await;

    let stargate_bridge_contract =
        StargateBridge::new(src_chain.stargate_bridge_address(), src_client);

    let mut tokens = merge_tokens_from_withdraw_request(dst_chain, event).await?;

    for (token_address, asset) in tokens.iter_mut() {
        let value_to_send = stargate_bridge_contract
            .prepareForTransport(
                *token_address,
                dst_chain as u32,
                event.user,
                asset.total_amount,
                asset.total_amount.checked_percent(98)?,
                Bytes::new(),
                0,
            )
            .call()
            .await?
            .valueToSend;

        asset.native_value = value_to_send;
    }

    Ok(tokens)
}
