CREATE TABLE IF NOT EXISTS feeds (
    id               INTEGER PRIMARY KEY,
    url              TEXT    NOT NULL UNIQUE,
    created_at       TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS seen_entries (
    feed_id     INTEGER NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    entry_id    TEXT    NOT NULL,
    seen_at     TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (feed_id, entry_id)
);
