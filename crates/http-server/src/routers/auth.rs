use axum::{Router, routing::get};

use crate::{extractors::state::AppState, handlers::auth};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/message", get(auth::get_msg::handler))
        .route("/auth/sign-in", get(auth::sign_in::handler))
}
