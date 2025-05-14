use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use shared::AppResult;
use web3::client::PublicClient;

pub struct ExternalPools {
    http_client: reqwest::Client,
    rpc_client: PublicClient,
}

pub struct ExternalPoolInfo {
    name: String,
    platform: String,
}

impl ExternalPools {
    async fn get_all_bridge_pool_info(&self, pool_name: &str) -> AppResult<()> {
        let pool = self
            .http_client
            .get("https://core.api.allbridgecoreapi.net/token-info?filter=all")
            .send()
            .await?
            .json::<HashMap<String, Chain>>()
            .await?;

        Ok(())
    }

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Token {
    name: String,
    pool_address: String,
    token_address: String,
    decimals: u8,
    symbol: String,
    pool_info: PoolInfo,
    fee_share: String,
    apr: String,
    apr7d: String,
    apr30d: String,
    lp_rate: String,
    cctp_address: Option<String>,
    cctp_v2_address: Option<String>,
    cctp_fee_share: Option<String>,
    cctp_v2_fee_share: Option<String>,
    flags: Flags,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PoolInfo {
    a_value: String,
    d_value: String,
    token_balance: String,
    v_usd_balance: String,
    total_lp_amount: String,
    acc_reward_per_share_p: String,
    p: u64,
}

#[derive(Debug, Deserialize)]
struct Flags {
    swap: bool,
    pool: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Chain {
    tokens: Vec<Token>,
    chain_id: u64,
    bridge_address: String,
    swap_address: String,
    transfer_time: HashMap<String, TransferTime>,
    confirmations: u64,
    tx_cost_amount: TxCostAmount,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferTime {
    allbridge: Option<u64>,
    wormhole: Option<u64>,
    cctp: Option<u64>,
    cctp_v2: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TxCostAmount {
    swap: String,
    transfer: String,
    max_amount: String,
}
