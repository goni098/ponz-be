use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::{AppResult, env::ENV};

use crate::error::{HttpException, HttpResult};

pub struct Redis(pub redis::aio::MultiplexedConnection);

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub redis_client: redis::Client,
}

impl<S> FromRequestParts<S> for Redis
where
    S: Send + Sync,
    redis::Client: FromRef<S>,
{
    type Rejection = HttpException;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> HttpResult<Self> {
        let connection = redis::Client::from_ref(state)
            .get_multiplexed_async_connection()
            .await?;

        Ok(Self(connection))
    }
}

impl AppState {
    pub async fn new() -> AppResult<AppState> {
        let mut opt = ConnectOptions::new(&ENV.db_url);

        opt.sqlx_logging(false);

        let db_conn = Database::connect(opt).await?;
        let redis_client = redis::Client::open(ENV.redis_url.as_str())?;

        Ok(Self {
            db_conn,
            redis_client,
        })
    }
}
