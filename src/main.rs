use anyhow::ensure;
use reqwest::Client;
use std::{fs, time::Duration};
use tracing::{Level, debug, info};
use tracing_subscriber::FmtSubscriber;

use crate::{
    config::{Config, FeedConfig},
    webhook::{DiscordEmbed, WebhookPayload},
};

mod config;
mod db;
mod webhook;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("loading config…");
    let config = fs::read_to_string("config.toml")?;
    debug!(config, "raw config toml");

    let config = toml::from_str::<Config>(&config)?;
    let feeds: Vec<_> = config.feeds.into_iter().map(|(_, v)| v).collect();

    info!("init db connection…");
    let conn = db::init_database("feeds.db")?;
    db::create_feeds(&conn, &feeds)?;

    info!("validating user agent");
    let user_agent = config.user_agent;
    ensure!(!user_agent.is_empty());

    info!("create http client");
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(30))
        .build()?;
    let mut handles = Vec::new();

    info!("start watching feeds");
    for feed in feeds {
        let client = client.clone();
        handles.push(tokio::spawn(watch_feed(feed, client)));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

async fn watch_feed(feed: FeedConfig, http_client: reqwest::Client) {
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

async fn fetch_entries(
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

async fn send_post(
    client: &Client,
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
