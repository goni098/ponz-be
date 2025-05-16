use std::collections::HashMap;

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
    balances: HashMap<Address, U256>,
) -> AppResult<HashMap<Address, Estimation>> {
    let src_client = get_public_client(src_chain).await;

    let stargate_bridge_contract =
        StargateBridge::new(src_chain.stargate_bridge_address(), src_client);

    let mut estimations = HashMap::new();

    for (token_address, balance) in balances.iter() {
        let prepare_for_transport = stargate_bridge_contract
            .prepareForTransport(
                *token_address,
                dst_chain as u32,
                user,
                *balance,
                balance.checked_percent(98)?,
                Bytes::new(),
                0,
            )
            .call()
            .await?;

        estimations.insert(*token_address, prepare_for_transport);
    }

    Ok(estimations)
}
