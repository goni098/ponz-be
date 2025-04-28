use serde_json::Value;
use shared::AppResult;
use web3::client::PublicClient;

pub struct Pool {
    http_client: reqwest::Client,
    _rpc_client: PublicClient,
}

impl Pool {
    pub async fn get(&self) -> AppResult<()> {
        let _pool = self
            .http_client
            .get("meme")
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(())
    }
}
