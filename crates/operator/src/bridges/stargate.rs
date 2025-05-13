use std::collections::HashMap;

use alloy::{
    primitives::{Address, Bytes},
    providers::Provider,
};
use alloy_chains::NamedChain;
use shared::{AppError, AppResult, util::CheckedPercent};
use web3::{
    DynChain,
    client::public_client,
    contracts::{router::Router::WithdrawRequest, stargate_bridge::StargateBridge},
};

use crate::withdraw::merge_asset::{TokenAsset, merge_tokens_from_withdraw_request};

pub async fn estimate_withdraw<P: Provider>(
    dst_chain: NamedChain,
    dst_client: P,
    event: &WithdrawRequest,
) -> AppResult<HashMap<Address, TokenAsset>> {
    let source_chain: NamedChain = event
        .chainId
        .to::<u64>()
        .try_into()
        .map_err(|_| AppError::Custom("Invalid chain id from WithdrawRequest event".into()))?;

    let source_client = public_client(source_chain);
    let stargate_bridge_contract =
        StargateBridge::new(source_chain.stargate_bridge_address(), source_client);

    let mut tokens = merge_tokens_from_withdraw_request(dst_client, event).await?;

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
