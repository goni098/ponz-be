use axum::{Router, response::Html, routing::get};
use extractors::state::AppState;
use shared::AppResult;
use tower_http::cors::CorsLayer;

mod error;
mod extractors;
mod handlers;
mod routers;

#[tokio::main]
async fn main() -> AppResult<()> {
    shared::logging::set_up("http_server=debug");

    let state = AppState::new().await?;

    let app = Router::new()
        .route("/", get(|| async { "🦀 hello !" }))
        .route(
            "/swagger/openapi.yml",
            get(|| async { include_str!("../openapi.yml") }),
        )
        .route(
            "/swagger",
            get(|| async { Html(include_str!("../openapi.html")) }),
        )
        .merge(routers::auth::routes())
        .merge(routers::users::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("🦀 server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
