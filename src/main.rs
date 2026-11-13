use reqwest::Client;
use std::fs;

use crate::{config::Config, webhook::WebhookPayload};

mod config;
mod webhook;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = fs::read_to_string("config.toml").unwrap();
    let config = toml::from_str::<Config>(&config).unwrap();

    let client = reqwest::Client::new();
    send_post(&client, &config).await?;
    Ok(())
}

async fn send_post(client: &Client, config: &Config) -> anyhow::Result<()> {
    let body = WebhookPayload {
        content: "hello world".to_string(),
    };
    let res = client
        .post(&config.discord.webhook_url)
        .json(&body)
        .send()
        .await?;
    println!("{res:#?}");
    Ok(())
}
