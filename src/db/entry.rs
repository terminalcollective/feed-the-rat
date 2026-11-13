#[derive(Debug, Clone)]
pub struct SeenEntry {
    pub feed_id: i64,
    pub entry_id: String,
    pub seen_at: String,
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
