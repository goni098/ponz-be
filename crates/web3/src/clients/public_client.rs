use std::collections::HashMap;

use alloy::providers::{
    Identity, ProviderBuilder, RootProvider,
    fillers::{ChainIdFiller, FillProvider, JoinFill},
};
use alloy_chains::NamedChain;
use tokio::sync::OnceCell;

use crate::DynChain;

pub type PublicClient = FillProvider<JoinFill<Identity, ChainIdFiller>, RootProvider>;

static PUBLIC_CLIENTS: OnceCell<HashMap<NamedChain, PublicClient>> = OnceCell::const_new();

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
