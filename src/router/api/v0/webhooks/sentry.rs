use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use send_wrapper::SendWrapper;
use serde_json::Value;
use worker::{Headers, Method, RequestInit};

use crate::state::AppState;

pub async fn handler(State(_state): State<AppState>, Json(payload): Json<Value>) -> Response {
    let message = build_discord_message(&payload);
    let result = SendWrapper::new(forward_to_discord(&message)).await;

    if let Err(err) = result {
        worker::console_error!("failed to forward sentry webhook to discord: {err}");
        return (StatusCode::BAD_GATEWAY, "failed to forward to discord").into_response();
    }

    (StatusCode::OK, "ok").into_response()
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

async fn forward_to_discord(body: &Value) -> worker::Result<()> {
    let headers = Headers::new();
    headers.set("content-type", "application/json")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(worker::wasm_bindgen::JsValue::from_str(
            &body.to_string(),
        )));

    // let request = WorkerRequest::new_with_init(webhook_url.as_str(), &init)?;
    // let mut response = Fetch::Request(request).send().await?;

    // if response.status_code() >= 400 {
    //     let text = response.text().await.unwrap_or_default();
    //     return Err(WorkerError::RustError(format!(
    //         "discord responded {}: {text}",
    //         response.status_code()
    //     )));
    // }

    Ok(())
}
