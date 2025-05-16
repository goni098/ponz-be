use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
use shared::AppResult;

mod allbridge;
mod balancer;
mod compund;

pub struct ExternalPoolsService {
    http_client: reqwest::Client,
    db: DatabaseConnection,
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
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            db,
        }
    }

    pub async fn find_top_choices(&self) -> AppResult<Vec<ExternalPoolInfo>> {
        let mut all_pools = vec![];

        let mut allbridge_pools = vec![];
        let mut balancer_pools_info = vec![];

        let supported_pool = repositories::pool::find_all_supported(&self.db).await?;

        for pool in supported_pool {
            if pool.platform == "allbridge" {
                allbridge_pools.push(pool);
            } else if pool.platform == "balancer" {
                balancer_pools_info
                    .push(balancer::fectch_pool_info(&self.http_client, &pool).await?);
            }
        }

        let allbridge_pools_info =
            allbridge::fetch_pools_info(&self.http_client, &allbridge_pools).await?;

        all_pools.extend(allbridge_pools_info);
        all_pools.extend(balancer_pools_info);

        all_pools.sort_by(|l, r| r.apr.total_cmp(&l.apr));

        Ok(all_pools)
    }
}
