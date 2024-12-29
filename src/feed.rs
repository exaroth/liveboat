use bs58::encode as bs58_encode;
use std::cmp::Reverse;
use std::fmt;

use libnewsboat::matchable::Matchable;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::feed_item::FeedItem;

const MAX_TRUNCATED_FEED_ITEMS: usize = 50;

/// Representation for single feed as retrieved from database.
/// Used for storing both url and query based feeds.
#[derive(Clone)]
pub struct Feed {
    id: String,
    pub title: String,
    display_title: String,
    url: String,
    pub feedlink: String,
    pub items: Vec<FeedItem>,
    hidden: bool,
    pub tags: Vec<String>,

    _is_query: bool,
    _order_idx: usize,
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

            _is_query: false,
            _order_idx: 0,
            _sorted: false,
        };
    }

    /// Initialize empty query feed, these feeds are composite of other feeds
    /// and filter params and are missing most of the feed parameters.
    pub fn init_query_feed(title: String, line_no: usize) -> Feed {
        Feed {
            id: bs58_encode(&title).into_string(),
            title: title.clone(),
            display_title: title.clone(),
            url: String::new(),
            feedlink: String::new(),
            items: Vec::new(),
            hidden: false,
            tags: Vec::new(),

            _is_query: true,
            _order_idx: line_no,
            _sorted: false,
        }
    }

    /// Add new article to the list of feed items.
    pub fn add_item(&mut self, item: FeedItem) {
        self.items.push(item)
    }

    /// Sort feed items from newest to oldest.
    pub fn sort_items(&mut self) {
        self.items.sort_by_key(|w| Reverse(w.date()));
        self._sorted = true
    }

    /// Compact list of articles to either 50 or week max so
    /// that we dont have to load all the articles at the same time.
    pub fn truncate_items(&mut self) {
        if self.items.len() <= MAX_TRUNCATED_FEED_ITEMS {
            return;
        }
        let items = self.items.clone();
        let last_week_items = items
            .into_iter()
            .filter(|i| i.age() <= 7)
            .collect::<Vec<FeedItem>>();
        if last_week_items.len() >= MAX_TRUNCATED_FEED_ITEMS {
            self.items = last_week_items;
            return;
        }
        self.items = self.items[0..MAX_TRUNCATED_FEED_ITEMS].to_vec();
    }

    /// Update feed with data retrieved from urls file.
    pub fn update_with_url_data(
        &mut self,
        tags: Vec<String>,
        hidden: bool,
        title_override: Option<String>,
        line_no: usize,
    ) {
        self.tags = tags;
        self.hidden = hidden;
        self._order_idx = line_no;
        if let Some(title) = title_override {
            self.display_title = title;
        }
    }

    pub fn url(&self) -> &String {
        return &self.url;
    }

    #[allow(dead_code)]
    pub fn title(&self) -> &String {
        return &self.display_title;
    }

    #[allow(dead_code)]
    pub fn display_title(&self) -> &String {
        return &self.display_title;
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
    pub fn order_idx(&self) -> &usize {
        return &self._order_idx;
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
                    // empty.
                    return Some(String::new());
                }
                if !self.is_sorted() {
                    panic!("Matcher called against unsorted feed")
                }
                return Some(format!("{:?}", self.items[0].age()));
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
        let mut state = serializer.serialize_struct("Feed", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("displayTitle", &self.display_title)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("feedLink", &self.feedlink)?;
        state.serialize_field("is_query", &self._is_query)?;
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
            .field("is_query", &self._is_query)
            .field("is_sorted", &self._sorted)
            .field("_order_idx", &self._order_idx)
            .finish()
    }
}

/// Compact version of the feed used for processing in feed lists.
#[derive(serde::Serialize)]
pub struct FeedCompact {
    id: String,
    title: String,
    display_title: String,
    url: String,
    feedlink: String,
    hidden: bool,
    is_query: bool,
    tags: Vec<String>,
    num_items: usize,
}

impl FeedCompact {
    fn from_feed(f: &Feed) -> FeedCompact {
        return FeedCompact {
            id: f.id.clone(),
            title: f.title().clone(),
            display_title: f.display_title.clone(),
            url: f.url.clone(),
            feedlink: f.feedlink.clone(),
            hidden: f.hidden,
            is_query: f._is_query,
            tags: f.tags.clone(),
            num_items: f.items.len(),
        };
    }
}

/// Representation of feed list.
#[derive(serde::Serialize)]
pub struct FeedList {
    feeds: Vec<FeedCompact>,
}

impl FeedList {
    pub fn new() -> FeedList {
        let f = FeedList { feeds: Vec::new() };
        return f;
    }

    #[allow(dead_code)]
    pub fn from_vec(v: Vec<Feed>) -> FeedList {
        let mut f = FeedList::new();
        for item in v {
            f.feeds.push(FeedCompact::from_feed(&item));
        }
        return f;
    }

    pub fn add_feed(&mut self, f: &Feed) {
        self.feeds.push(FeedCompact::from_feed(f))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::feed_item::*;

    #[test]
    fn test_adding_feed_item() {
        let item = FeedItem::new("item1", "http://test.com", "", "", 123456, false, "", 1);
        let mut f = Feed::init(
            "http://example.com".to_string(),
            "Url feed1".to_string(),
            "http://testfeed.com".to_string(),
        );
        assert_eq!(0, f.items.len());
        f.add_item(item);
        assert_eq!(1, f.items.len());
    }
    #[test]
    fn test_sorting_feed_items() {
        let item1 = FeedItem::new("item1", "http://test.com", "", "", 950000000, false, "", 1);
        let item2 = FeedItem::new("item2", "http://test.com", "", "", 960000000, false, "", 2);
        let item3 = FeedItem::new("item3", "http://test.com", "", "", 970000000, false, "", 3);
        let mut f = Feed::init(
            "http://example.com".to_string(),
            "Url feed1".to_string(),
            "http://testfeed.com".to_string(),
        );
        f.add_item(item1);
        f.add_item(item2);
        f.add_item(item3);
        assert_eq!(f._sorted, false);
        assert_eq!(1, f.items[0].guid());
        assert_eq!(2, f.items[1].guid());
        assert_eq!(3, f.items[2].guid());
        f.sort_items();
        assert_eq!(f._sorted, true);
        assert_eq!(3, f.items[0].guid());
        assert_eq!(2, f.items[1].guid());
        assert_eq!(1, f.items[2].guid());
    }

    #[test]
    fn test_matching_basic_content() {
        let mut f = Feed::init(
            "http://example.com".to_string(),
            "Url feed".to_string(),
            "http://testfeed.com".to_string(),
        );
        f.tags.push("dev".to_string());
        f.tags.push("tech".to_string());
        f.tags.push("news".to_string());
        let mut attr = f.attribute_value("feedtitle");
        assert!(attr.is_some());
        assert_eq!(attr, Some("Url feed".to_string()));
        attr = f.attribute_value("feedlink");
        assert!(attr.is_some());
        assert_eq!(attr, Some("http://testfeed.com".to_string()));
        attr = f.attribute_value("rssurl");
        assert!(attr.is_some());
        assert_eq!(attr, Some("http://example.com".to_string()));
        attr = f.attribute_value("tags");
        assert_eq!(attr, Some("dev tech news".to_string()));
    }

    #[test]
    fn test_matching_feed_articles() {
        let mut f = Feed::init("".to_string(), "".to_string(), "".to_string());
        let item1 = FeedItem::new("item1", "http://test.com", "", "", 970000000, false, "", 1);
        let item2 = FeedItem::new("item2", "http://test.com", "", "", 960000000, true, "", 2);
        let item3 = FeedItem::new("item3", "http://test.com", "", "", 950000000, true, "", 3);
        f.add_item(item1);
        f.add_item(item3);
        f.add_item(item2);
        f.sort_items();

        let mut attr = f.attribute_value("total_count");
        assert_eq!(Some("3".to_string()), attr);
        attr = f.attribute_value("unread_count");
        assert_eq!(Some("2".to_string()), attr);
        attr = f.attribute_value("latest_article_age");
        assert_eq!(Some("8842".to_string()), attr);
    }

    #[test]
    fn test_matching_nonexistent_attrs() {
        let f = Feed::init(
            "http://example.com".to_string(),
            "Url feed".to_string(),
            "http://testfeed.com".to_string(),
        );
        let attr = f.attribute_value("nonexistent");
        assert!(attr.is_none());
    }
}
