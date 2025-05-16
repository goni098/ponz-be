use alloy::primitives::{Address, Bytes, U256};
use alloy_chains::NamedChain;
use shared::{AppResult, util::CheckedPercent};
use web3::{
    DynChain,
    client::get_public_client,
    contracts::stargate_bridge::StargateBridge::{self, prepareForTransportReturn},
};

type Estimation = prepareForTransportReturn;

pub async fn estimate_withdraw(
    src_chain: NamedChain,
    dst_chain: NamedChain,
    user: Address,
    token_address: Address,
    token_balance: U256,
) -> AppResult<Estimation> {
    let src_client = get_public_client(src_chain).await;

    let stargate_bridge_contract =
        StargateBridge::new(src_chain.stargate_bridge_address(), src_client);

    let prepare_for_transport = stargate_bridge_contract
        .prepareForTransport(
            token_address,
            dst_chain as u32,
            user,
            token_balance,
            token_balance.checked_percent(98)?,
            Bytes::new(),
            0,
        )
        .call()
        .await?;

    Ok(prepare_for_transport)
}
