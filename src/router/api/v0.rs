use axum::Router;

use crate::state::AppState;

pub mod webhooks;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().nest("/webhooks", webhooks::routes())
}
