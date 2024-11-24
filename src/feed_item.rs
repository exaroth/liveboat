use crate::feed::Feed;
use chrono::{DateTime, Local, TimeDelta};
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
    date: i64,
    unread: bool,
    content: String,
    guid: i64,
    enc_url: Option<String>,
    enc_mime: Option<String>,
    flags: Option<String>,
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
            content: row.get(8)?,
            guid: row.get(9)?,
            enc_url: row.get(10)?,
            enc_mime: row.get(11)?,
            flags: row.get(12)?,
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

    pub fn date(&self) -> i64 {
        return self.date;
    }
	pub fn age(&self) -> i64 {
		let now = Local::now();
		if let Some(d) = DateTime::from_timestamp(self.date, 0) {
			let delta = now.signed_duration_since(d);
			return delta.num_days()
		};
		return 0
	}
	pub fn is_unread(&self) -> bool {
		return self.unread
	}
}

impl Matchable for FeedItem {
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
            "date" => Some(format!("{}", self.date)),
            "age" => Some(format!("{}", self.age())),
            "content" => Some(self.content.clone()),
            "guid" => Some(format!("{}", self.guid)),
            "enclosure_url" => opt_attr_val(&self.enc_url),
            "enclosure_type" => opt_attr_val(&self.enc_mime),
            "flags" => opt_attr_val(&self.flags),
			// TODO (kw)
            "articleindex" => Some(String::new()),
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

fn opt_attr_val(attr: &Option<String>) -> Option<String> {
    if let Some(a) = attr {
        return Some(a.clone());
    }
    return Some(String::new());
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
