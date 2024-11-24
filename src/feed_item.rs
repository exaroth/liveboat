use crate::feed::Feed;
use libnewsboat::matchable::Matchable;
use rusqlite::Error as SQLiteError;
use rusqlite::Row;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FeedItem {
    feed_url: String,
    title: String,
    url: String,
    author: String,
    desc: String,
    date: u32,
    unread: bool,
    pub feed_ptr: Option<Arc<RefCell<Feed>>>,
}

impl FeedItem {
    pub fn from_db_row(row: &Row) -> Result<FeedItem, SQLiteError> {
        let feed_item = FeedItem {
            feed_url: row.get(0)?,
            title: row.get(2)?,
            url: row.get(3)?,
            author: row.get(4)?,
            desc: row.get(5)?,
            date: row.get(6)?,
            unread: row.get(7)?,
            feed_ptr: None,
        };
        Ok(feed_item)
    }
    pub fn feed_url(&self) -> &String {
        return &self.feed_url;
    }
    pub fn set_ptr(&mut self, f_p: Arc<RefCell<Feed>>) {
        self.feed_ptr = Some(f_p)
    }

    pub fn date(&self) -> u32 {
        return self.date;
    }
}

impl Matchable for FeedItem {
    // TODO
    fn attribute_value(&self, attr: &str) -> Option<String> {
        match attr {
            "title" => Some(self.title.clone()),
            "link" => Some(self.url.clone()),
            "author" => Some(self.author.clone()),
            "unread" => {
                let unread = if self.unread {
                    String::from("yes")
                } else {
                    String::from("no")
                };
                Some(unread)
            }
            // "age" => Some(String::from("0")),
            // TODO
            // "date" => Some(String::new()),
            // "guid" => Some(String::new()),
            // "enclosure_url" => Some(String::new()),
            // "enclosure_type" => Some(String::new()),
            // "flags" => Some(String::new()),
            // "articleindex" => Some(String::new()),

            // "content" => Some(String::new()),
            _ => {
                if let Some(feed) = &self.feed_ptr {
                    feed.borrow().attribute_value(attr)
                } else {
                    None
                }
            }
        }
    }
}

impl Serialize for FeedItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Feed", 2)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("url", &self.url)?;
        state.end()
    }
}
