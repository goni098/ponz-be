use alloy::providers::{ProviderBuilder, RootProvider, WsConnect};
use alloy_chains::NamedChain;
use shared::AppResult;

use crate::DynChain;

pub type WsClient = RootProvider;

pub async fn create_ws_client(chain: NamedChain) -> AppResult<RootProvider> {
    let ws = WsConnect::new(chain.ws_url().as_str());

    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_ws(ws)
        .await?;

    Ok(provider)
}
