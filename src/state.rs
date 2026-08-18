use anyhow::Result;
use worker::Env;

use crate::sink::discord::Discord;

#[derive(Clone)]
pub struct AppState {
    pub discord: Discord,
    pub sentry_integration_token: String,
}

impl AppState {
    pub fn new(env: Env) -> Result<Self> {
        let discord_webhook_url = env
            .secret("DISCORD_WEBHOOK_URL")
            .map_err(|err| anyhow::anyhow!(err.to_string()))?
            .to_string();
        let sentry_integration_token = env
            .secret("SENTRY_INTEGRATION_TOKEN")
            .map_err(|err| anyhow::anyhow!(err.to_string()))?
            .to_string();

        Ok(Self {
            discord: Discord::new(discord_webhook_url),
            sentry_integration_token,
        })
    }
}
