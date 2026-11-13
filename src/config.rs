use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub user_agent: String,
    pub feeds: HashMap<String, FeedConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
}

#[derive(Debug, Deserialize)]
pub struct FeedConfig {
    pub url: String,
    pub interval: u64,
}
