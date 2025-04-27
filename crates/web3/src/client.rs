use std::sync::LazyLock;

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
use solana_client::client_error::reqwest::Url;
use strum::IntoEnumIterator;

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

pub fn public_client(rpc_url: Url) -> PublicClient {
    ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_http(rpc_url)
}

pub fn wallet_client(rpc_url: Url, wallet: EthereumWallet) -> WalletClient {
    ProviderBuilder::new().wallet(wallet).connect_http(rpc_url)
}

pub struct Clients {
    sepolia: PublicClient,
    main: PublicClient,
    base: PublicClient,
}

pub static CLIENTS: LazyLock<Clients> = LazyLock::new(|| Clients {
    base: public_client("rpc_url".parse().unwrap()),
    main: public_client("rpc_url".parse().unwrap()),
    sepolia: public_client("rpc_url".parse().unwrap()),
});

impl Clients {
    pub fn get_by_chain(&self, chain: NamedChain) -> &PublicClient {
        let chain = NamedChain::iter()
            .find(|c| *c == chain)
            .unwrap_or(NamedChain::Base);

        match chain {
            NamedChain::Base => &self.base,
            NamedChain::Sepolia => &self.sepolia,
            NamedChain::Mainnet => &self.main,
            _ => &self.sepolia,
        }
    }

    pub fn all(&self) -> [(&PublicClient, NamedChain); 3] {
        [
            (&self.base, NamedChain::Base),
            (&self.main, NamedChain::Mainnet),
            (&self.sepolia, NamedChain::ScrollSepolia),
        ]
    }
}
