use axum::{Router, routing::get};

use crate::{extractors::state::AppState, hanlders::users::me};

pub fn routes() -> Router<AppState> {
    Router::new().route("/users/me", get(me::handler))
}
