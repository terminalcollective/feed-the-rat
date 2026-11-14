use anyhow::ensure;
use std::{fs, time::Duration};
use tracing::{Level, debug, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::config::Config;

mod config;
mod db;
mod utils;
mod webhook;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,feed_the_rat=debug"));

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        .with_env_filter(filter)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("loading config…");
    let config = fs::read_to_string("config.toml")?;
    debug!(config, "raw config toml");

    let config = toml::from_str::<Config>(&config)?;
    let feeds: Vec<_> = config.feeds.into_iter().map(|(_, v)| v).collect();

    info!("validating user agent");
    let user_agent = config.user_agent;
    ensure!(!user_agent.is_empty());

    info!("create http client");
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(30))
        .build()?;

    info!("init db connection…");
    let conn = db::init_database("feeds.db")?;

    info!("try fill db…");
    db::fill_database(&conn, &feeds, &client).await?;

    let mut handles = Vec::new();

    info!("start watching feeds");
    for feed in feeds {
        let client = client.clone();
        handles.push(tokio::spawn(utils::watch_feed(feed, client)));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
