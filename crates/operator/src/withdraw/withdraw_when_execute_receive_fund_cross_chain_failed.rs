use alloy_chains::NamedChain;
use shared::AppResult;
use web3::{
    client::WalletClient,
    contracts::stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed,
};

pub async fn withdraw_when_execute_receive_fund_cross_chain_failed(
    chain: NamedChain,
    wallet_client: &WalletClient,
    event: ExecuteReceiveFundCrossChainFailed,
) -> AppResult<()> {
    Ok(())
}
