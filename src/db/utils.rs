use rusqlite::Connection;
use std::path::Path;
use tracing::info;

use crate::{db::entry::EntryId, utils::fetch_entries};

pub fn init_database(db_path: impl AsRef<Path>) -> anyhow::Result<Connection> {
    let conn = Connection::open(db_path)?;
    let schema_sql = include_str!("schema.sql");

    conn.pragma_update(None, "foreign_keys", true)?;
    conn.execute_batch(schema_sql)?;

    Ok(conn)
}

pub async fn fill_database(
    conn: &Connection,
    feeds: &[crate::config::FeedConfig],
    http_client: &reqwest::Client,
) -> anyhow::Result<()> {
    for feed in feeds {
        let new_feed = crate::db::create_feed(conn, &feed.url)?;
        info!(
            "created new feed {} {} {}",
            new_feed.id, new_feed.url, new_feed.created_at
        );

        match fetch_entries(&feed.url, &http_client).await {
            Ok(entries) => {
                info!("fetched {} entries", entries.len());
                for entry in entries {
                    let new_entry = crate::db::create_entry(
                        conn,
                        EntryId {
                            feed_id: new_feed.id,
                            entry_id: entry.id,
                        },
                    )?;
                    info!(
                        "created new entry {} {} {}",
                        new_entry.feed_id, new_entry.entry_id, new_entry.seen_at
                    );
                }
            }
            Err(err) => eprintln!("[{}] failed to fetch: {err}", feed.url),
        };
    }

    Ok(())
}
