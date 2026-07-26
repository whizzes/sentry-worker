use axum::{http::StatusCode, response::IntoResponse};
use tower_service::Service;
use worker::{Context, Env, HttpRequest, Result, event};

use crate::state::AppState;

use self::router::api::routes;

mod router;
mod sink;
mod state;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let Ok(state) = AppState::new(env) else {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create an instance of AppState",
        )
            .into_response());
    };

    Ok(routes(state).call(req).await?)
}
