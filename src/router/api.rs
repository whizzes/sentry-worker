use axum::Router;

use crate::state::AppState;

pub mod v0;

pub fn routes(state: AppState) -> Router {
    Router::<AppState>::new()
        .nest("/api/v0", v0::routes())
        .with_state(state)
}
