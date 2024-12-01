use crate::feed::Feed;
use crate::opts::Options;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Representation of default context to be passed
/// when rendering index template.
#[derive(serde::Serialize)]
pub struct Context<'a> {
    build_time: u64,
    feeds: Vec<Feed>,
    query_feeds: &'a Vec<Feed>,
    options: &'a Options,
}

impl<'a> Context<'a> {
    pub fn init(
        url_feeds: &'a Vec<Arc<RefCell<Feed>>>,
        query_feeds: &'a Vec<Feed>,
        options: &'a Options,
    ) -> Context<'a> {
        let mut feeds = Vec::new();
        for f in url_feeds {
            let item = <RefCell<Feed> as Clone>::clone(&f).into_inner();
            if item.is_hidden() || item.is_empty() {
                continue;
            }
            feeds.push(item);
        }

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let build_time = since_the_epoch.as_secs();

        Context {
            feeds,
            query_feeds,
            build_time,
            options,
        }
    }
}
