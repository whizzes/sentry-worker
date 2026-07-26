use axum::{Router, routing::post};

use crate::state::AppState;

mod sentry;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().route("/sentry", post(sentry::handler))
}
