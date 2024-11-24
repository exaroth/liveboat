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

    pub fn is_sorted(&self) -> bool {
        self._sorted
    }
    
    pub fn is_empty(&self) -> bool {
        return self.items.len() == 0
    }

}

impl Matchable for Feed {
    fn attribute_value(&self, attr: &str) -> Option<String> {
        match attr {
            "feedtitle" => Some(self.title.clone()),
            "rssurl" => Some(self.url.clone()),
            "total_count" => Some(format!("{}", self.items.len())),
            "tags" => Some(self.tags.join(" ")),
            "latest_article_age" => {
                if self.is_empty() {
                    // Should never occur since we dont render
                    // empty 
                    return Some(String::new())
                }
                if !self.is_sorted() {
                    panic!("Matcher called against unsorted feed")
                }
                return Some(format!("{}", self.items[0].age()))
            },
            "unread_count" => {
                let n = self.items.iter().filter(|i| i.is_unread()).count();
                Some(format!("{}", n))
            }
            // TODO
            "feedlink" => Some(String::new()),
            "feedindex" => Some(String::new()),
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
