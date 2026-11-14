use crate::{
    config::{Config, FeedConfig},
    webhook::{self, DiscordEmbed, WebhookPayload},
};
use std::time::Duration;

pub async fn watch_feed(feed: FeedConfig, http_client: reqwest::Client) {
    let interval = Duration::from_secs(feed.interval.clone());
    let mut ticker = tokio::time::interval(interval);

    loop {
        ticker.tick().await;

        match fetch_entries(&feed.url, &http_client).await {
            Ok(entries) => {
                let latest = entries.first();
                todo!("implement feed process logic");
            }
            Err(err) => eprintln!("[{}] failed to fetch: {err}", feed.url),
        };
    }
}

pub async fn fetch_entries(
    feed_url: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<Vec<feed_rs::model::Entry>> {
    let xml = http_client
        .get(feed_url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let feed = feed_rs::parser::parse(xml.as_bytes())?;
    Ok(feed.entries)
}

pub async fn send_post(
    client: &reqwest::Client,
    config: &Config,
    entry: &feed_rs::model::Entry,
) -> anyhow::Result<()> {
    let content = entry.summary.clone().unwrap().content.to_string();
    let body = WebhookPayload {
        embeds: Some(vec![DiscordEmbed {
            title: Some("Hello World".to_string()),
            embed_type: None,
            description: Some(content),
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
