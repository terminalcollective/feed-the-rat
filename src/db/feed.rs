use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct Feed {
    pub id: i64,
    pub url: String,
    pub created_at: String,
}

impl Feed {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            url: row.get("url")?,
            created_at: row.get("created_at")?,
        })
    }
}

pub fn create_feed(conn: &Connection, url: &str) -> rusqlite::Result<Feed> {
    conn.execute(
        "INSERT OR IGNORE INTO feeds (url) VALUES (?1)",
        rusqlite::params![url],
    )?;

    let id: i64 = conn.query_row(
        "SELECT id FROM feeds WHERE url = ?1",
        rusqlite::params![url],
        |row| row.get(0),
    )?;

    get_feed_by_id(conn, id)
}

pub fn get_feed_by_id(conn: &Connection, id: i64) -> rusqlite::Result<Feed> {
    conn.query_row(
        "SELECT id, url, created_at FROM feeds WHERE id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(Feed {
                id: row.get(0)?,
                url: row.get(1)?,
                created_at: row.get(2)?,
            })
        },
    )
}
