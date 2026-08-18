use serde_json::Value;
use worker::{Fetch, Headers, Method, Request, RequestInit};

#[derive(Clone)]
pub struct Discord {
    webhook_url: String,
}

impl Discord {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }

    pub async fn send(&self, message: &Value) -> worker::Result<()> {
        let headers = Headers::new();
        headers.set("content-type", "application/json")?;

        let mut init = RequestInit::new();
        init.with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(worker::wasm_bindgen::JsValue::from_str(
                &message.to_string(),
            )));

        let request = Request::new_with_init(&self.webhook_url, &init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() >= 400 {
            let text = response.text().await.unwrap_or_default();
            return Err(worker::Error::RustError(format!(
                "discord responded {}: {text}",
                response.status_code()
            )));
        }

        Ok(())
    }
}
