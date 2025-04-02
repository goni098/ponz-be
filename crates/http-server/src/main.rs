use axum::{Router, routing::get};
use extractors::state::AppState;
use shared::Rlt;
use tower_http::cors::CorsLayer;

mod error;
mod extractors;
mod handlers;
mod routers;

#[tokio::main]
async fn main() -> Rlt<()> {
    shared::logging::set_up("http_server=debug");

    let db_url = shared::env::read_env("DATABASE_URL")?;
    let redis_url = shared::env::read_env("REDIS_URL")?;
    let solana_rpc_url = shared::env::read_env("SOLANA_RPC_URL")?;

    let state = AppState::new(db_url, redis_url, solana_rpc_url).await?;

    let app = Router::new()
        .route("/", get(|| async { "🦀 hello !" }))
        .merge(routers::auth::routes())
        .merge(routers::users::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("🦀 server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
