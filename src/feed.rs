use libnewsboat::matchable::Matchable;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

use crate::feed_item::FeedItem;

pub struct Feed {
    title: String,
    url: String,
    pub items: Vec<FeedItem>,
    hidden: bool,
    tags: Vec<String>,
    is_query: bool,

    _sorted: bool,
}

impl Feed {
    pub fn add_item(&mut self, item: FeedItem) {
        self.items.push(item)
    }

    /// Sort feed items from newest to oldest.
    pub fn sort_items(&mut self) {
        self.items.sort_by(|a, b| a.date().cmp(&b.date()));
        self._sorted = true
    }

    pub fn init(title: String, url: String) -> Feed {
        return Feed {
            title: title,
            url: url,
            hidden: false,
            items: Vec::new(),
            tags: Vec::new(),
            is_query: false,

            _sorted: false,
        };
    }

    /// Initialize empty query feed, these feeds are composite of other feeds
    /// and filter params and are missing most of the feed parameters.
    pub fn init_query_feed(title: String) -> Feed {
        Feed {
            title: title,
            url: String::new(),
            items: Vec::new(),
            hidden: false,
            tags: Vec::new(),
            is_query: true,
            _sorted: false,
        }
    }

    pub fn update_with_url_data(&mut self, tags: Vec<String>, hidden: bool) {
        self.tags = tags;
        self.hidden = hidden;
    }

    pub fn url(&self) -> &String {
        return &self.url;
    }
}

impl Matchable for Feed {
    fn attribute_value(&self, attr: &str) -> Option<String> {
        match attr {
            "feedtitle" => Some(self.title.clone()),
            "rssurl" => Some(self.url.clone()),
            "total_count" => Some(format!("{}", self.items.len())),
            "tags" => Some(self.tags.join(" ")),
            // TODO
            // "unread_count" => Some(String::new()),
            // "latest_article_age" => Some(String::new()),
            // "feedlink" => Some(self.),
            // "feedindex" => Some(String::new()),
            // TODO not implemented as these are not stored
            // in cache.
            "description" => Some(String::new()),
            "feeddate" => Some(String::new()),
            _ => None,
        }
    }
}

impl Serialize for Feed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Feed", 2)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("items", &self.items)?;
        state.end()
    }
}

impl fmt::Debug for Feed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Feed")
            .field("url", &self.url)
            .field("title", &self.title)
            .field("num_items", &self.items.len())
            .field("tags", &self.tags)
            .field("hidden", &self.hidden)
            .field("is_query", &self.is_query)
            .finish()
    }
}
