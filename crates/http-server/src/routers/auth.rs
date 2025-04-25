use axum::{
    Router,
    routing::{get, post},
};

use crate::{extractors::state::AppState, handlers::auth};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/message", get(auth::get_msg::handler))
        .route("/auth/renew-tokens", post(auth::renew_tokens::handler))
        .route("/auth/sign-in", post(auth::sign_in::handler))
}
