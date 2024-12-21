#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::feed::Feed;
use crate::feed_item::FeedItem;

use anyhow::Result;
use log::{info, trace};
use std::path::Path;

use rusqlite::Error as SQLiteError;
use rusqlite::{params_from_iter, Connection, Rows};

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

#[cfg_attr(test, automock)]
pub trait Connector {
    fn get_feed_items(&self, days_back: u64) -> Result<Vec<FeedItem>>;
    fn get_feeds(&self, urls: Vec<String>) -> Result<Vec<Feed>>;
}

pub struct DBConnector {
    conn: Connection,
}

impl DBConnector {
    pub fn init(db_path: &Path) -> Result<DBConnector> {
        let connector = DBConnector {
            conn: Connection::open(db_path)?,
        };
        Ok(connector)
    }
    /// Instantiate feed objects based on the rows retrieved from db.
    fn load_feed_items(&self, rows: &mut Rows<'_>) -> Result<Vec<FeedItem>, SQLiteError> {
        let mut results: Vec<FeedItem> = Vec::new();
        while let Some(row) = rows.next()? {
            let feed_item = FeedItem::from_db_row(row)?;
            trace!("load_feed_items:: Adding Feed item: {}", feed_item);
            results.push(feed_item);
        }
        Ok(results)
    }
}

impl Connector for DBConnector {
    /// Retrieve article data from sqlite db.
    fn get_feed_items(&self, days_back: u64) -> Result<Vec<FeedItem>> {
        let mut stmt = self.conn.prepare(FEED_ITEMS_SQL)?;
        info!(
            "Prepared statement for feed retrieval: {}",
            stmt.expanded_sql().unwrap()
        );
        // NOTE: we cant interpolate days integer directly with rusql
        let days_s = format!("-{} days", days_back);
        info!("Day threshold param == {}", days_s);
        let mut results = stmt.query(rusqlite::named_params! {"$days": days_s})?;
        let results = self.load_feed_items(&mut results)?;
        Ok(results)
    }

    /// Retrieve feed information from sqlite db, we do it only for feeds defined in urls file.
    fn get_feeds(&self, urls: Vec<String>) -> Result<Vec<Feed>> {
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
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(params_from_iter(urls.iter()), |row| {
            let f = Feed::init(row.get(0)?, row.get(1)?, row.get(2)?);
            Ok(f)
        })?;
        let mut result = Vec::new();
        for r in rows {
            match r {
                Ok(f) => result.push(f),
                Err(e) => return Err(e.into()),
            }
        }
        trace!("Retrieved feeds: {}", format!("{:?}", result));
        Ok(result)
    }
}
