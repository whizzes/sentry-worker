use anyhow::Result;
use worker::Env;

#[derive(Clone)]
pub struct WebHooks {}

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub webhooks: WebHooks,
}

impl AppState {
    pub fn new(_env: Env) -> Result<Self> {
        Ok(Self {
            webhooks: WebHooks {},
        })
    }
}
