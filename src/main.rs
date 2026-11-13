use reqwest::Client;
use std::fs;

use crate::{
    config::Config,
    webhook::{DiscordEmbed, WebhookPayload},
};

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
        embeds: Some(vec![DiscordEmbed {
            title: Some("Hello World".to_string()),
            embed_type: None,
            description: Some("some desc".to_string()),
            url: Some("https://terminalcollective.org/".to_string()),
            timestamp: None,
            color: Some(3395962),
            footer: Some(webhook::EmbedFooter {
                text: "Terminal Collective".to_string(),
                icon_url: Some(
                    "https://avatars.githubusercontent.com/u/193274422?s=200&v=4".to_string(),
                ),
                proxy_icon_url: None,
            }),
            image: Some(webhook::EmbedImage {
                url: "https://raw.githubusercontent.com/coko7/vegapull/refs/heads/main/assets/vegapull_logo.png"
                    .to_string(),
            }),
            thumbnail: None,
            author: None,
            fields: None,
        }]),
        content: None,
        username: None,
        avatar_url: None,
        tts: None,
    };

    let res = client
        .post(&config.discord.webhook_url)
        .json(&body)
        .send()
        .await?;

    println!("{res:#?}");
    Ok(())
}
