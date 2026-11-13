use rusqlite::Connection;
use std::path::Path;

pub fn init_database(db_path: impl AsRef<Path>) -> anyhow::Result<Connection> {
    let conn = Connection::open(db_path)?;
    let schema_sql = include_str!("schema.sql");

    conn.pragma_update(None, "foreign_keys", true)?;
    conn.execute_batch(schema_sql)?;

    Ok(conn)
}

pub fn create_feeds(conn: &Connection, feeds: &[crate::config::FeedConfig]) -> anyhow::Result<()> {
    for feed in feeds {
        crate::db::create_feed(conn, &feed.url)?;
    }
    Ok(())
}
