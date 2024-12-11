use crate::feed::Feed;
use crate::feed_item::FeedItem;
use log::{info, trace};
use std::error::Error;
use std::path::Path;

use rusqlite::Error as SQLiteError;
use rusqlite::{params_from_iter, Connection, Result, Rows};

const FEED_ITEMS_SQL: &str = "SELECT 
    feed.rssurl AS feed_url,
    feed.title AS feed_title,
    items.title AS item_title,
    items.url AS item_url,
    items.author AS item_author,
    items.enclosure_description AS item_desc,
    items.pubDate AS pub_date,
    items.unread AS unread,
    items.content AS content,
    items.id AS guid,
    items.enclosure_url AS enc_url,
    items.enclosure_description_mime_type AS enc_mime_type,
    items.flags AS flags
FROM rss_item AS items
JOIN rss_feed AS feed ON feed.rssurl = items.feedurl
WHERE datetime(items.pubDate, 'unixepoch') >= datetime('now', $days )
AND items.deleted=0
";

fn get_db_connection(path: &Path) -> Result<Connection, SQLiteError> {
    let conn = Connection::open(path)?;
    Ok(conn)
}

/// Instantiate feed objects based on the rows retrieved from db.
fn load_feed_items(rows: &mut Rows<'_>) -> Result<Vec<FeedItem>, SQLiteError> {
    let mut results: Vec<FeedItem> = Vec::new();
    while let Some(row) = rows.next()? {
        let feed_item = FeedItem::from_db_row(row)?;
        trace!("load_feed_items:: Adding Feed item: {}", feed_item);
        results.push(feed_item);
    }
    Ok(results)
}

/// Retrieve article data from sqlite db.
pub fn get_feed_item_data(db_path: &Path, days_back: u64) -> Result<Vec<FeedItem>, Box<dyn Error>> {
    let conn = get_db_connection(db_path)?;
    let mut stmt = conn.prepare(FEED_ITEMS_SQL)?;
    info!(
        "Prepared statement for feed retrieval: {}",
        stmt.expanded_sql().unwrap()
    );
    // NOTE: we cant interpolate days integer directly with rusql
    let days_s = format!("-{} days", days_back);
    info!("Day threshold param == {}", days_s);
    let mut results = stmt.query(rusqlite::named_params! {"$days": days_s})?;
    let results = load_feed_items(&mut results)?;
    Ok(results)
}

/// Retrieve feed information from sqlite db, we do it only for feeds defined in urls file.
pub fn get_feed_data(db_path: &Path, urls: Vec<String>) -> Result<Vec<Feed>, SQLiteError> {
    let repeat_vars = |c| {
        assert_ne!(c, 0);
        let mut s = "?,".repeat(c);
        s.pop();
        s
    };
    let sql = format!(
        "SELECT rssurl, title, url FROM rss_feed where rssurl in ({});",
        repeat_vars(urls.len())
    );
    trace!("Feed retrieval SQL: {}", sql);
    let conn = get_db_connection(db_path)?;
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(urls.iter()), |row| {
        let f = Feed::init(row.get(0)?, row.get(1)?, row.get(2)?);
        Ok(f)
    })?;
    let mut result = Vec::new();
    for r in rows {
        match r {
            Ok(f) => result.push(f),
            Err(e) => return Err(e),
        }
    }
    trace!("Retrieved feeds: {}", format!("{:?}", result));
    Ok(result)
}
