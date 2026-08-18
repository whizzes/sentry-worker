use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use send_wrapper::SendWrapper;
use serde_json::Value;

use crate::state::AppState;

pub async fn handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Response {
    if !is_authorized(&headers, &state.sentry_integration_token) {
        return (StatusCode::UNAUTHORIZED, "invalid or missing token").into_response();
    }

    let message = build_discord_message(&payload);
    let result = SendWrapper::new(state.discord.send(&message)).await;

    if let Err(err) = result {
        worker::console_error!("failed to forward sentry webhook to discord: {err}");
        return (StatusCode::BAD_GATEWAY, "failed to forward to discord").into_response();
    }

    (StatusCode::OK, "ok").into_response()
}

fn is_authorized(headers: &HeaderMap, expected_token: &str) -> bool {
    let Some(value) = headers.get(header::AUTHORIZATION) else {
        return false;
    };

    let Ok(value) = value.to_str() else {
        return false;
    };

    let parts: Vec<&str> = value.split(' ').collect();

    if parts.len() != 2 {
        return false;
    }

    if !parts[0].eq_ignore_ascii_case("bearer") {
        return false;
    }

    let token = parts[1];

    token == expected_token
}

fn build_discord_message(payload: &Value) -> Value {
    let action = payload
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("unknown");

    let issue = payload.get("data").and_then(|data| data.get("issue"));

    let title = issue
        .and_then(|issue| issue.get("title"))
        .and_then(Value::as_str)
        .unwrap_or("Sentry event");

    let culprit = issue
        .and_then(|issue| issue.get("culprit"))
        .and_then(Value::as_str);

    let level = issue
        .and_then(|issue| issue.get("level"))
        .and_then(Value::as_str)
        .unwrap_or("error");

    let project = issue
        .and_then(|issue| issue.get("project"))
        .and_then(|project| project.get("name"))
        .and_then(Value::as_str);

    let permalink = issue
        .and_then(|issue| issue.get("web_url").or_else(|| issue.get("permalink")))
        .and_then(Value::as_str);

    let mut description = format!("**Action:** {action}\n**Level:** {level}");
    if let Some(project) = project {
        description.push_str(&format!("\n**Project:** {project}"));
    }
    if let Some(culprit) = culprit {
        description.push_str(&format!("\n**Culprit:** {culprit}"));
    }

    let mut embed = serde_json::json!({
        "title": title,
        "description": description,
        "color": 15_158_332,
    });

    if let Some(permalink) = permalink {
        embed["url"] = Value::String(permalink.to_string());
    }

    serde_json::json!({ "embeds": [embed] })
}
