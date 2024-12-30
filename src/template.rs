use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

use crate::errors::FilesystemError;
use crate::feed::Feed;
use crate::opts::Options;

pub const TEMPLATE_CONFIG_FNAME: &str = "config.toml";

fn default_template_settings() -> HashMap<String, String> {
    return HashMap::new();
}

fn default_builder() -> String {
    String::from("spa")
}

/// This is representation of per template
/// configuration which is attached to every template
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TemplateConfig {
    pub version: String,
    #[serde(default = "default_builder")]
    #[allow(dead_code)]
    pub builder: String,
    #[serde(default = "default_template_settings")]
    pub template_settings: HashMap<String, String>,
}
impl TemplateConfig {
    /// Instantiate template settings from TOML file.
    pub fn get_config_for_template(tpl_path: &Path) -> Result<TemplateConfig> {
        let cfg_path = tpl_path.join(TEMPLATE_CONFIG_FNAME);
        if !cfg_path.exists() {
            return Err(FilesystemError::PathDoesNotExist(cfg_path).into());
        }
        let raw = read_to_string(cfg_path)?;
        let cfg = toml::from_str(raw.as_str())?;
        return Ok(cfg);
    }
}

pub trait Context {
    fn feeds(&self) -> &Vec<Feed>;
    #[allow(dead_code)]
    fn options(&self) -> &Options;
    fn build_time(&self) -> u64;
}

/// Representation of default context to be passed
/// when rendering index template.
#[derive(serde::Serialize)]
pub struct SimpleContext<'a> {
    feeds: Vec<Feed>,
    options: &'a Options,
    build_time: u64,
    template_settings: &'a HashMap<String, String>,
    template_version: String,
}

impl<'a> Context for SimpleContext<'a> {
    fn feeds(&self) -> &Vec<Feed> {
        return &self.feeds;
    }
    fn options(&self) -> &Options {
        return &self.options;
    }
    fn build_time(&self) -> u64 {
        return self.build_time;
    }
}

impl<'a> SimpleContext<'a> {
    pub fn init(
        url_feeds: &'a Vec<Arc<RefCell<Feed>>>,
        query_feeds: &'a Vec<Feed>,
        options: &'a Options,
        template_settings: &'a HashMap<String, String>,
        template_version: String,
    ) -> SimpleContext<'a> {
        let mut feeds = Vec::new();
        for f in url_feeds {
            let item = <RefCell<Feed> as Clone>::clone(&f).into_inner();
            feeds.push(item);
        }
        for q_feed in query_feeds {
            if q_feed.is_empty() {
                continue;
            }
            feeds.push(q_feed.clone());
        }
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let build_time = since_the_epoch.as_secs();

        feeds.sort_by(|a, b| a.order_idx().cmp(b.order_idx()));
        SimpleContext {
            feeds,
            options,
            build_time,
            template_settings,
            template_version,
        }
    }
}

impl fmt::Display for SimpleContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ctx::
            feed_num {}:
            opts: {}
            build_time: {}",
            self.feeds.len(),
            self.options,
            self.build_time,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::feed_item::*;

    #[test]
    fn test_processing_feeds_for_template() {
        let mut feeds = Vec::new();

        // Feed non hidden with and item
        let item1 = FeedItem::new("item1", "http://test.com", "", "",  123456, false, "", 1);

        let mut f1 = Feed::init(
            "http://example.com".to_string(),
            "Url feed1".to_string(),
            "http://testfeed.com".to_string(),
        );

        f1.update_with_url_data(Vec::new(), false, None, 4);
        f1.add_item(item1.clone());
        feeds.push(Arc::new(RefCell::new(f1)));

        //  Hidden url feed with items
        let mut f2 = Feed::init(
            "http://example2.com".to_string(),
            "Url feed2".to_string(),
            "http://testfeed.com".to_string(),
        );
        f2.update_with_url_data(Vec::new(), true, None, 2);
        f2.add_item(item1.clone());
        feeds.push(Arc::new(RefCell::new(f2)));
        // Feed with no items, non hidden
        let mut f3 = Feed::init(
            "http://example3.com".to_string(),
            "Url feed3".to_string(),
            "http://testfeed.com".to_string(),
        );
        f3.update_with_url_data(Vec::new(), false, None, 1);
        feeds.push(Arc::new(RefCell::new(f3)));

        let mut query_feeds = Vec::new();
        // Query feed with no items
        query_feeds.push(Feed::init_query_feed(String::from("Query feed1"), 10));
        // Query feed with items
        let mut qfeed = Feed::init_query_feed(String::from("Query feed2"), 2);
        qfeed.add_item(item1.clone());
        query_feeds.push(qfeed);

        let opts = Options::default();
        let settings = HashMap::new();
        let ctx = SimpleContext::init(
            &feeds,
            &query_feeds,
            &opts,
            &settings,
            String::from("1.0.0"),
        );

        assert_eq!(4, ctx.feeds.len());
        let titles = ctx.feeds.into_iter().map(|f| f.title().clone()).collect::<Vec<String>>();
        assert!(titles.contains(&"Url feed1".to_string()));
        assert!(titles.contains(&"Url feed2".to_string()));
        assert!(titles.contains(&"Url feed3".to_string()));
        assert!(titles.contains(&"Query feed2".to_string()));
    }
}
