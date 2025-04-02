use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::Rlt;
use solana_client::nonblocking::rpc_client::RpcClient;

use crate::error::{ServerErr, ServerRlt};

pub type SolanaClient = Arc<RpcClient>;

pub struct Redis(pub redis::aio::MultiplexedConnection);

#[derive(FromRef, Clone)]
pub struct AppState {
    pub solana_client: SolanaClient,
    pub db_conn: DatabaseConnection,
    pub redis_client: redis::Client,
}

impl<S> FromRequestParts<S> for Redis
where
    S: Send + Sync,
    redis::Client: FromRef<S>,
{
    type Rejection = ServerErr;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> ServerRlt<Self> {
        let connection = redis::Client::from_ref(state)
            .get_multiplexed_async_connection()
            .await?;

        Ok(Self(connection))
    }
}

impl AppState {
    pub async fn new(db_url: String, redis_url: String, solana_rpc_url: String) -> Rlt<AppState> {
        let mut opt = ConnectOptions::new(db_url);

        opt.sqlx_logging(false);

        let db_conn = Database::connect(opt).await?;
        let solana_client = Arc::new(RpcClient::new(solana_rpc_url));
        let redis_client = redis::Client::open(redis_url)?;

        Ok(Self {
            db_conn,
            solana_client,
            redis_client,
        })
    }
}
