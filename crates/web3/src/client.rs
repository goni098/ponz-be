use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, RootProvider,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
};
use alloy_chains::NamedChain;
use shared::env::ENV;
use url::Url;

pub type PublicClient = RootProvider;

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

pub fn public_client(chain: NamedChain) -> PublicClient {
    ProviderBuilder::new()
        .disable_recommended_fillers()
        .with_chain(chain)
        .connect_http(resolve_rpc_url(chain))
}

pub fn wallet_client(chain: NamedChain, wallet: EthereumWallet) -> WalletClient {
    ProviderBuilder::new()
        .wallet(wallet)
        .with_chain(chain)
        .connect_http(resolve_rpc_url(chain))
}

fn resolve_rpc_url(chain: NamedChain) -> Url {
    match chain {
        NamedChain::Sepolia => ENV.sepolia_rpc_url.clone(),
        NamedChain::Base => ENV.base_rpc_url.clone(),
        _ => panic!("unsupported chain {}", chain),
    }
}
