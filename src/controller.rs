use clap::Parser;
use std::cell::RefCell;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

use libnewsboat::utils as libutils;
use rusqlite::Error as SQLiteError;
use rusqlite::{params_from_iter, Connection, Result, Rows};

use serde::Serialize;
use crate::args::Args;
use crate::config::Paths;
use crate::feed::Feed;
use crate::feed_item::FeedItem;
use crate::opts::Options;
use crate::urls::UrlReader;

#[derive(Debug)]
pub struct Controller {
    paths: Paths,
    options: Options,
    url_reader: UrlReader,
}

const FEED_ITEMS_SQL: &str = "SELECT 
    feed.rssurl AS feed_url,
    feed.title AS feed_title,
    items.title AS item_title,
    items.url AS item_url,
    items.author AS item_author,
    items.enclosure_description AS item_desc,
    items.pubDate AS pub_date,
    items.unread AS unread
FROM rss_item AS items
JOIN rss_feed AS feed ON feed.rssurl = items.feedurl
WHERE datetime(items.pubDate, 'unixepoch') >= datetime('now', '-10 days')
AND items.deleted=0;
";

impl Controller {
    pub fn init() -> Result<Controller, Box<dyn Error>> {
        let args = Args::parse();
        let paths = Paths::new(args)?;
        let opts = Options::init(paths.config_file())?;
        let url_reader = UrlReader::init(paths.url_file());

        let ctrl = Controller {
            paths: paths,
            options: opts,
            url_reader: url_reader,
        };
        Ok(ctrl)
    }

    pub fn process(&self) -> Result<(), Box<dyn Error>> {
        let feed_items = self.get_feed_item_data()?;
        let feeds = self.load_feeds()?;
        for item in feed_items {
            if let Some(f) = feeds.iter().find(|f| f.borrow().url() == item.feed_url()) {
                let mut i = item.clone();
                i.set_ptr(Arc::clone(f));
                f.borrow_mut().add_item(i);
                continue;
            }
        }
        let q_feeds = self.get_query_feeds(feeds)?;
        for f in q_feeds {
            println!("{:?}", serde_json::to_string(&f).unwrap());
        }
        Ok(())
    }

    fn get_query_feeds(&self, feeds: Vec<Arc<RefCell<Feed>>>) -> Result<Vec<Feed>, Box<dyn Error>> {
        let mut result = Vec::new();
        let query_feeds = &self.url_reader.get_query_urls()?;
        for query_f in query_feeds {
            let mut q = Feed::init_query_feed(query_f.title.clone());
            for f in &feeds {
                for i in &f.borrow().items {
                    match query_f.matcher.matches(i) {
                        // TODO, check MatcherError
                        Err(_) => continue,
                        Ok(matches) => {
                            if matches {
                                q.add_item(i.clone())
                            }
                        }
                    }
                }
            }
            result.push(q)
        }
        Ok(result)
    }

    fn get_db_connection(&self) -> Result<Connection, SQLiteError> {
        let conn = Connection::open(&self.paths.cache_file())?;
        Ok(conn)
    }

    fn load_feed_items(&self, rows: &mut Rows<'_>) -> Result<Vec<FeedItem>, SQLiteError> {
        let mut results: Vec<FeedItem> = Vec::new();
        while let Some(row) = rows.next()? {
            let feed_item = FeedItem::from_db_row(row)?;
            results.push(feed_item);
        }
        Ok(results)
    }

    fn load_feeds(&self) -> Result<Vec<Arc<RefCell<Feed>>>, Box<dyn Error>> {
        let url_feeds = self.url_reader.get_url_feeds();
        let urls = url_feeds.iter().map(|u| u.url.clone()).collect();
        let feed_data = self.get_feed_data(urls)?;
        for f in &feed_data {
            if let Some(url_feed) = url_feeds.iter().find(|u| &u.url == f.borrow().url()) {
                f.borrow_mut()
                    .update_with_url_data(url_feed.tags.clone(), url_feed.hidden)
            }
        }
        Ok(feed_data)
    }

    fn get_feed_item_data(&self) -> Result<Vec<FeedItem>, Box<dyn Error>> {
        let conn = &self.get_db_connection()?;
        let mut stmt = conn.prepare(FEED_ITEMS_SQL)?;
        let mut r = stmt.query([])?;
        let results = self.load_feed_items(&mut r)?;
        Ok(results)
    }

    fn get_feed_data(&self, urls: Vec<String>) -> Result<Vec<Arc<RefCell<Feed>>>, SQLiteError> {
        let repeat_vars = |c| {
            assert_ne!(c, 0);
            let mut s = "?,".repeat(c);
            // Remove trailing comma
            s.pop();
            s
        };
        let sql = format!(
            "SELECT rssurl, title FROM rss_feed where rssurl in ({});",
            repeat_vars(urls.len())
        );
        let conn = &self.get_db_connection()?;
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_from_iter(urls.iter()), |row| {
            let f = Feed::init(row.get(1)?, row.get(0)?);
            Ok(f)
        })?;
        let mut result = Vec::new();
        for r in rows {
            match r {
                Ok(f) => result.push(Arc::new(RefCell::new(f))),
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }
}
