use std::collections::HashMap;

use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, RootProvider, WsConnect,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
    signers::local::PrivateKeySigner,
};
use alloy_chains::NamedChain;
use shared::{AppResult, secret::get_secret};
use tokio::sync::OnceCell;

use crate::DynChain;

pub type PublicClient = FillProvider<JoinFill<Identity, ChainIdFiller>, RootProvider>;

pub type WsClient = RootProvider;

pub type WalletClient = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

static PUBLIC_CLIENTS: OnceCell<HashMap<NamedChain, PublicClient>> = OnceCell::const_new();
static WALLET_CLIENTS: OnceCell<HashMap<NamedChain, WalletClient>> = OnceCell::const_new();

pub fn create_public_client(chain: NamedChain) -> PublicClient {
    ProviderBuilder::new()
        .disable_recommended_fillers()
        .with_chain_id(chain as u64)
        .connect_http(chain.rpc_url())
}

pub async fn get_public_client(chain: NamedChain) -> &'static PublicClient {
    PUBLIC_CLIENTS
        .get_or_init(|| async {
            let mut clients = HashMap::new();

            for supported_chain in NamedChain::supported_chains() {
                let client = ProviderBuilder::new()
                    .disable_recommended_fillers()
                    .with_chain_id(supported_chain as u64)
                    .connect_http(supported_chain.rpc_url());

                clients.insert(supported_chain, client);
            }
            clients
        })
        .await
        .get(&chain)
        .unwrap_or_else(|| panic!("Public client has not set for chain {}", chain))
}

pub async fn get_wallet_client(chain: NamedChain) -> &'static WalletClient {
    WALLET_CLIENTS
        .get_or_init(|| async {
            let mut clients = HashMap::new();

            let operator_pk = &get_secret().await.operator_pk;

            let operator = operator_pk
                .parse::<PrivateKeySigner>()
                .expect("Invalid operator_pk");

            dbg!(operator.address());

            for supported_chain in NamedChain::supported_chains() {
                let client = ProviderBuilder::new()
                    .wallet(EthereumWallet::new(operator.clone()))
                    .with_chain(chain)
                    .connect_http(chain.rpc_url());

                clients.insert(supported_chain, client);
            }
            clients
        })
        .await
        .get(&chain)
        .unwrap_or_else(|| panic!("Wallet client has not set for chain {}", chain))
}

pub async fn create_ws_client(chain: NamedChain) -> AppResult<RootProvider> {
    let ws = WsConnect::new(chain.ws_url().as_str());

    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_ws(ws)
        .await?;

    Ok(provider)
}
