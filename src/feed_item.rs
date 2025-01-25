use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;

use libnewsboat::matchable::Matchable;
use rss::Item as RSSItem;
use rss::{Category, ItemBuilder, Source};
use rusqlite::Error as SQLiteError;
use rusqlite::Row;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use chrono::DateTime;

use crate::feed::Feed;
use crate::utils::now;

/// Container for storing and operating
/// on the newsboat article items.
#[derive(Debug, Clone)]
pub struct FeedItem {
    /// Direct url of the feed associated with given article
    /// NOTE: for query urls, this will point to source URL feed
    /// not query one (which do not have url associated with them)
    feed_url: String,
    /// Title of the feed as retrieved from RSS feed.
    title: String,
    /// URL for the article.
    url: String,
    /// Author of the article.
    author: String,
    /// Timestamp of the article.
    date: i64,
    /// Whether or not article has been
    /// marked as read (or not) by Newsboat.
    unread: bool,
    /// Content of the article, will include
    /// html tags if these were provided by source
    /// feed.
    content: String,
    /// Raw content text of the article, will exclude
    /// any associated HTML tags.
    text: Option<String>,
    /// Native Newsboat guid (id of the article in db)
    guid: i64,
    /// Length of the content (includes raw text of the article
    /// excluding tags)
    content_length: usize,
    /// Optional link to comment site for given article.
    comments_url: Option<String>,
    /// Url to media associated with the article, eg. mp3 file,
    /// youtube link etc.
    enc_url: Option<String>,
    /// Mimetype of enclosure url associated with the article.
    enc_mime: Option<String>,
    /// unused at the moment
    flags: Option<String>,
    /// Pointer of feed associated with given article,
    /// for query feeds will point to source url feed.
    pub feed_ptr: Option<Arc<RefCell<Feed>>>,
}

impl FeedItem {
    /// Initialize new article item from db row.
    pub fn from_db_row(row: &Row) -> Result<FeedItem, SQLiteError> {
        let feed_item = FeedItem {
            feed_url: row.get(0)?,
            title: row.get(2)?,
            url: row.get(3)?,
            author: row.get(4)?,
            date: row.get(5)?,
            unread: row.get(6)?,
            content: row.get(7)?,
            guid: row.get(8)?,
            enc_url: row.get(9)?,
            enc_mime: row.get(10)?,
            flags: row.get(11)?,
            content_length: 0,
            text: None,
            comments_url: None,
            feed_ptr: None,
        };
        Ok(feed_item)
    }

    /// Direct url of the feed associated with given article
    /// NOTE: for query urls, this will point to source URL feed
    /// not query one (which do not have url associated with them)
    pub fn feed_url(&self) -> &String {
        return &self.feed_url;
    }

    /// Title of the article as retrieved from RSS feed.
    #[allow(dead_code)]
    pub fn title(&self) -> &String {
        return &self.title;
    }

    /// URL for the article.
    pub fn url(&self) -> &String {
        return &self.url;
    }

    /// Author of the article.
    #[allow(dead_code)]
    pub fn author(&self) -> &String {
        return &self.author;
    }

    /// Timestamp of the article.
    pub fn date(&self) -> i64 {
        return self.date;
    }

    /// Whether or not article has been
    /// marked as read (or not) by Newsboat.
    #[allow(dead_code)]
    pub fn unread(&self) -> bool {
        return self.unread;
    }

    /// Content of the article, will include
    /// html tags if these were provided by source
    /// feed.
    pub fn content(&self) -> &String {
        return &self.content;
    }

    /// Native Newsboat guid (id of the article in db)
    pub fn guid(&self) -> &i64 {
        return &self.guid;
    }

    /// Optional link to comment site for given article.
    #[allow(dead_code)]
    pub fn comments_url(&self) -> &Option<String> {
        return &self.comments_url;
    }

    /// Raw content text of the article, will exclude
    /// any associated HTML tags.
    #[allow(dead_code)]
    pub fn text(&self) -> &Option<String> {
        return &self.text;
    }

    /// Length of the content (includes raw text of the article
    /// excluding tags)
    #[allow(dead_code)]
    pub fn content_length(&self) -> usize {
        return self.content_length;
    }

    /// Return age of the article (in days).
    pub fn age(&self) -> i64 {
        let tnow = now();
        if let Some(d) = DateTime::from_timestamp(self.date, 0) {
            let delta = tnow.signed_duration_since(d).num_days();
            // This will happen if rss channel exposes wrong date (in the future)
            if delta < 0 {
                return 0;
            }
            return delta;
        };
        return 0;
    }

    /// Whether or not article has been
    /// marked as read (or not) by Newsboat.
    pub fn is_unread(&self) -> bool {
        return self.unread;
    }

    /// Url to media associated with the article, eg. mp3 file,
    /// youtube link etc.
    #[allow(dead_code)]
    pub fn enc_url(&self) -> &Option<String> {
        return &self.enc_url;
    }

    /// Mimetype of enclosure url associated with the article.
    #[allow(dead_code)]
    pub fn enc_mime(&self) -> &Option<String> {
        return &self.enc_mime;
    }

    /// Set a pointer to feed associated with the article.
    pub fn set_ptr(&mut self, f_p: Arc<RefCell<Feed>>) {
        self.feed_ptr = Some(f_p)
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content
    }

    pub fn set_content_length(&mut self, size: usize) {
        self.content_length = size
    }

    pub fn set_comments_url(&mut self, url: String) {
        self.comments_url = Some(url)
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text)
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url
    }

    /// Convert date ts assigned to feed item to datetime string
    fn get_rfc_dt(&self) -> String {
        let dt = DateTime::from_timestamp(self.date, 0);
        if dt.is_none() {
            return String::new();
        }
        return dt.unwrap().to_rfc2822();
    }

    /// Create new RSS Item based on feed item data.
    pub fn to_rss_item(self, include_content: bool) -> RSSItem {
        let mut item = ItemBuilder::default()
            .title(self.title.clone())
            .link(self.url.clone())
            .author(self.author.clone())
            .pub_date(self.get_rfc_dt())
            .build();

        if include_content {
            item.set_content(self.text.clone())
        }
        if self.feed_ptr.is_some() {
            let f = self.feed_ptr.unwrap();
            item.set_source(Some(Source {
                title: Some(f.borrow().display_title().clone()),
                url: f.borrow().feedlink().clone(),
            }));
            let mut categories = Vec::new();
            for cat in f.borrow().tags.clone() {
                categories.push(Category {
                    name: cat,
                    // TODO
                    domain: None,
                })
            }
            item.set_categories(categories)
        }
        return item;
    }

    #[allow(dead_code)]
    pub fn new(
        title: &str,
        url: &str,
        feed_url: &str,
        author: &str,
        date: i64,
        unread: bool,
        content: &str,
        guid: i64,
    ) -> FeedItem {
        return FeedItem {
            title: title.to_string(),
            url: url.to_string(),
            feed_url: feed_url.to_string(),
            author: author.to_string(),
            date: date,
            unread: unread,
            content: content.to_string(),
            guid: guid,
            enc_url: None,
            enc_mime: None,
            flags: None,
            text: None,
            feed_ptr: None,
            comments_url: None,
            content_length: 0,
        };
    }
}

impl Matchable for FeedItem {
    /// Returns filter attributes which are used by newsboat
    /// to generate query feeds.
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
            // This index is generated by the newsboat when rendering article
            // list so we skip it during filtering.
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

impl fmt::Display for FeedItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FeedItem::
            feed url {}:
            title: {}
            url: {}
            date: {}
            unread: {}",
            self.feed_url, self.title, self.url, self.date, self.unread,
        )
    }
}

/// Utility used for processing Optional, string based attrs
/// so we can still utilize them even if not set. Empty attrs
/// are ignored by the filter parser.
fn opt_attr_val(attr: &Option<String>) -> Option<String> {
    if let Some(a) = attr {
        return Some(a.clone());
    }
    return Some(String::new());
}

impl Serialize for FeedItem {
    /// JSON serialization attributes.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FeedItem", 12)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("unread", &self.unread)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("contentLength", &self.content_length)?;
        state.serialize_field("flags", &self.flags)?;
        state.serialize_field("enclosureUrl", &self.enc_url)?;
        state.serialize_field("enclosureMime", &self.enc_mime)?;
        state.serialize_field("commentsUrl", &self.comments_url)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_matching_basic_feed_item_attrs() {
        let item = FeedItem::new(
            "item1",
            "http://test.com",
            "",
            "exaroth",
            123456,
            false,
            "Test content",
            1,
        );
        let mut attr = item.attribute_value("title");
        assert_eq!(Some("item1".to_string()), attr);
        attr = item.attribute_value("link");
        assert_eq!(Some("http://test.com".to_string()), attr);
        attr = item.attribute_value("author");
        assert_eq!(Some("exaroth".to_string()), attr);
        attr = item.attribute_value("unread");
        assert_eq!(Some("no".to_string()), attr);
        attr = item.attribute_value("date");
        assert_eq!(Some("123456".to_string()), attr);
        attr = item.attribute_value("age");
        assert_eq!(Some("20067".to_string()), attr);
        attr = item.attribute_value("content");
        assert_eq!(Some("Test content".to_string()), attr);
        attr = item.attribute_value("guid");
        assert_eq!(Some("1".to_string()), attr);
    }

    #[test]
    fn test_matching_optional_feed_item_attrs() {
        let mut item = FeedItem::new(
            "item1",
            "http://test.com",
            "",
            "exaroth",
            123456,
            false,
            "Test content",
            1,
        );
        let mut attr = item.attribute_value("enclosure_url");
        assert_eq!(Some("".to_string()), attr);
        attr = item.attribute_value("enclosure_type");
        assert_eq!(Some("".to_string()), attr);
        attr = item.attribute_value("flags");
        assert_eq!(Some("".to_string()), attr);
        item.enc_url = Some("http://test.com".to_string());
        item.enc_mime = Some("video/mp4".to_string());
        item.flags = Some("flag1 flag2".to_string());
        attr = item.attribute_value("enclosure_url");
        assert_eq!(Some("http://test.com".to_string()), attr);
        attr = item.attribute_value("enclosure_type");
        assert_eq!(Some("video/mp4".to_string()), attr);
        attr = item.attribute_value("flags");
        assert_eq!(Some("flag1 flag2".to_string()), attr);
    }

    #[test]
    fn test_falling_back_to_feed_attribute_if_not_found() {
        let f = Arc::new(RefCell::new(Feed::init(
            "http://feed.com".to_string(),
            "Feed".to_string(),
            "http://feedlink.com".to_string(),
        )));
        let mut item = FeedItem::new("item1", "http://test.com", "", "", 970000000, false, "", 1);
        item.set_ptr(Arc::clone(&f));
        f.borrow_mut().add_item(item);
        let mut attr = f.borrow().items[0].attribute_value("feedlink");
        assert_eq!(Some("http://feedlink.com".to_string()), attr);
        attr = f.borrow().items[0].attribute_value("rssurl");
        assert_eq!(Some("http://feed.com".to_string()), attr);
        attr = f.borrow().items[0].attribute_value("nonexistent");
        assert!(attr.is_none());
    }

    #[test]
    fn test_retrieving_age_of_the_article() {
        let mut item = FeedItem::new(
            "item1",
            "http://test.com",
            "",
            "exaroth",
            1766842490,
            false,
            "Test content",
            1,
        );
        // Test for invalid dates (in the future)
        let mut age = item.attribute_value("age");
        assert_eq!(age, Some("0".to_string()));
        // Test current day
        item = FeedItem::new(
            "item1",
            "http://test.com",
            "",
            "exaroth",
            1733974900,
            false,
            "Test content",
            1,
        );
        age = item.attribute_value("age");
        assert_eq!(age, Some("0".to_string()));
        // 2 days backwards
        item = FeedItem::new(
            "item1",
            "http://test.com",
            "",
            "exaroth",
            1733800000,
            false,
            "Test content",
            1,
        );
        age = item.attribute_value("age");
        assert_eq!(age, Some("2".to_string()));
    }
}
