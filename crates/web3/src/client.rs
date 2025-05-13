use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, RootProvider, WsConnect,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
};
use alloy_chains::NamedChain;
use shared::AppResult;

use crate::DynChain;

pub type PublicClient = FillProvider<JoinFill<Identity, ChainIdFiller>, RootProvider>;

pub type WsClient = RootProvider;

pub type WalletClient = FillProvider<
    JoinFill<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        ChainIdFiller,
    >,
    RootProvider,
>;

pub fn public_client(chain: NamedChain) -> PublicClient {
    ProviderBuilder::new()
        .disable_recommended_fillers()
        .with_chain_id(chain as u64)
        .connect_http(chain.rpc_url())
}

pub async fn ws_client(chain: NamedChain) -> AppResult<RootProvider> {
    let ws = WsConnect::new(chain.ws_url().as_str());
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_ws(ws)
        .await?;

    Ok(provider)
}

pub fn wallet_client(chain: NamedChain, wallet: EthereumWallet) -> WalletClient {
    ProviderBuilder::new()
        .wallet(wallet)
        .with_chain_id(chain as u64)
        .connect_http(chain.rpc_url())
}
