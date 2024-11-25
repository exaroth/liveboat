use crate::feed::Feed;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(serde::Serialize)]
pub struct Context<'a> {
    feeds: Vec<Feed>,
    query_feeds: &'a Vec<Feed>,
}

impl <'a>Context<'a> {
    
    pub fn init(url_feeds: &'a Vec<Arc<RefCell<Feed>>>, query_feeds: &'a Vec<Feed>) -> Context<'a> {
        let mut feeds = Vec::new();
        for f in url_feeds {
            let item = <RefCell<Feed> as Clone>::clone(&f).into_inner();
            feeds.push(item)
        }
        Context{feeds: feeds, query_feeds: query_feeds}
    }
}
