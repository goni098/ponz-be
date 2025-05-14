use alloy_chains::NamedChain;
use database::models::SupportedPool;
use shared::AppResult;

mod allbridge;

pub struct ExternalPoolsService {
    http_client: reqwest::Client,
}

#[derive(Debug)]
pub struct ExternalPoolInfo {
    pub platform: String,
    pub name: String,
    pub pool_address: String,
    pub token_address: String,
    pub tvl: f64,
    pub apr: f64,
    pub apr_7d: f64,
    pub apr_30d: f64,
    pub chain: NamedChain,
    pub strategy_address: String,
}

impl ExternalPoolsService {
    pub async fn find_all(&self, pools: &[SupportedPool]) -> AppResult<Vec<ExternalPoolInfo>> {
        let mut all_pools = vec![];

        let allbridge_pools = allbridge::fetch_pools_info(&self.http_client, pools).await?;

        all_pools.extend(allbridge_pools);

        all_pools.sort_by(|l, r| r.apr.total_cmp(&l.apr));

        Ok(all_pools)
    }
}
