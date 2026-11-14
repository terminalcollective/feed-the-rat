use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct SeenEntry {
    pub feed_id: i64,
    pub entry_id: String,
    pub seen_at: String,
}

#[derive(Debug, Clone)]
pub struct EntryId {
    pub feed_id: i64,
    pub entry_id: String,
}

impl SeenEntry {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            feed_id: row.get("feed_id")?,
            entry_id: row.get("entry_id")?,
            seen_at: row.get("seen_at")?,
        })
    }
}

pub fn create_entry(conn: &Connection, id: EntryId) -> rusqlite::Result<SeenEntry> {
    conn.execute(
        "INSERT OR IGNORE INTO seen_entries (feed_id, entry_id) VALUES (?1, ?2)",
        rusqlite::params![id.feed_id, id.entry_id],
    )?;

    get_entry_by_id(conn, id)
}

pub fn get_entry_by_id(conn: &Connection, id: EntryId) -> rusqlite::Result<SeenEntry> {
    conn.query_row(
        "SELECT feed_id, entry_id, seen_at FROM seen_entries WHERE feed_id = ?1 AND entry_id = ?2",
        rusqlite::params![id.feed_id, id.entry_id],
        |row| {
            Ok(SeenEntry {
                feed_id: row.get(0)?,
                entry_id: row.get(1)?,
                seen_at: row.get(2)?,
            })
        },
    )
}
