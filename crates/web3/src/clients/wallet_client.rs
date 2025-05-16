use std::collections::HashMap;

use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, RootProvider,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
    signers::local::PrivateKeySigner,
};
use alloy_chains::NamedChain;
use shared::secret::get_secret;
use tokio::sync::OnceCell;

use crate::DynChain;

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

static WALLET_CLIENTS: OnceCell<HashMap<NamedChain, WalletClient>> = OnceCell::const_new();

pub async fn get_wallet_client(chain: NamedChain) -> &'static WalletClient {
    WALLET_CLIENTS
        .get_or_init(|| async {
            let mut clients = HashMap::new();

            let operator_pk = &get_secret().await.operator_pk;

            let operator = operator_pk
                .parse::<PrivateKeySigner>()
                .expect("Invalid operator_pk");

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
