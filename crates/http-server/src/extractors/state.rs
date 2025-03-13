use std::sync::Arc;

use axum_macros::FromRef;
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::Rlt;
use solana_client::nonblocking::rpc_client::RpcClient;

pub type SolanaClient = Arc<RpcClient>;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub solana_client: SolanaClient,
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new(db_url: String, solana_rpc_url: String) -> Rlt<AppState> {
        let mut opt = ConnectOptions::new(db_url);

        opt.sqlx_logging(false);

        let db = Database::connect(opt).await?;
        let solana_client = Arc::new(RpcClient::new(solana_rpc_url));

        Ok(Self { db, solana_client })
    }
}
