use bs58::encode as bs58_encode;
use libnewsboat::matchable::Matchable;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

use crate::feed_item::FeedItem;

// TODO, override title with url one
pub struct Feed {
    id: String,
    title: String,
    display_title: String,
    url: String,
    feedlink: String,
    pub items: Vec<FeedItem>,
    hidden: bool,
    tags: Vec<String>,
    is_query: bool,

    _sorted: bool,
}

impl Feed {
    pub fn init(url: String, title: String, feedlink: String) -> Feed {
        return Feed {
            id: bs58_encode(&url).into_string(),
            title: title.clone(),
            display_title: title.clone(),
            url: url,
            feedlink: feedlink,
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
        // TODO: add tags for the feed as well.
        Feed {
            id: bs58_encode(&title).into_string(),
            title: title.clone(),
            display_title: title.clone(),
            url: String::new(),
            feedlink: String::new(),
            items: Vec::new(),
            hidden: false,
            tags: Vec::new(),
            is_query: true,
            _sorted: false,
        }
    }

    /// Add new article to the list of feed items.
    pub fn add_item(&mut self, item: FeedItem) {
        self.items.push(item)
    }

    /// Sort feed items from newest to oldest.
    pub fn sort_items(&mut self) {
        self.items.sort_by(|a, b| a.date().cmp(&b.date()));
        self._sorted = true
    }

    /// Update feed with data retrieved from urls file.
    pub fn update_with_url_data(
        &mut self,
        tags: Vec<String>,
        hidden: bool,
        title_override: Option<String>,
    ) {
        self.tags = tags;
        self.hidden = hidden;
        if let Some(title) = title_override {
            self.display_title = title;
        }
    }

    pub fn url(&self) -> &String {
        return &self.url;
    }

    pub fn is_sorted(&self) -> bool {
        return self._sorted;
    }

    pub fn is_hidden(&self) -> bool {
        return self.hidden;
    }

    pub fn is_empty(&self) -> bool {
        return self.items.len() == 0;
    }
    pub fn id(&self) -> &String {
        return &self.id;
    }
}

impl Matchable for Feed {
    fn attribute_value(&self, attr: &str) -> Option<String> {
        match attr {
            "feedtitle" => Some(self.title.clone()),
            "rssurl" => Some(self.url.clone()),
            "feedlink" => Some(self.feedlink.clone()),
            "total_count" => Some(format!("{}", self.items.len())),
            "tags" => Some(self.tags.join(" ")),
            "latest_article_age" => {
                if self.is_empty() {
                    // Should never occur since we dont render
                    // empty
                    return Some(String::new());
                }
                if !self.is_sorted() {
                    panic!("Matcher called against unsorted feed")
                }
                return Some(format!("{}", self.items[0].age()));
            }
            "unread_count" => {
                let n = self.items.iter().filter(|i| i.is_unread()).count();
                Some(format!("{}", n))
            }
            // TODO
            "description" => Some(String::new()),
            "feeddate" => Some(String::new()),
            // feed index is generated when displaying feed by newsboat
            // and hence it can't be used. (kw)
            "feedindex" => Some(String::new()),
            _ => None,
        }
    }
}

impl Serialize for Feed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Feed", 5)?;
        // TODO: add feed id.
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("displayTitle", &self.display_title)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("feedLink", &self.feedlink)?;
        state.serialize_field("items", &self.items)?;
        state.end()
    }
}

impl fmt::Debug for Feed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Feed")
            .field("url", &self.url)
            .field("title", &self.title)
            .field("display_title", &self.title)
            .field("num_items", &self.items.len())
            .field("feedlink", &self.feedlink)
            .field("tags", &self.tags)
            .field("hidden", &self.hidden)
            .field("is_query", &self.is_query)
            .finish()
    }
}
