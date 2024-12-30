use chrono::Local;
use log::info;
use std::cmp::Reverse;
use std::collections::HashMap;

use opml::{Body, Head, Outline, OPML};
use rss::ChannelBuilder;

use crate::feed::Feed;
use crate::opts::Options;

/// Generate RSS channel for the liveboat page.
pub fn generate_rss_channel(opts: &Options, feeds: &Vec<Feed>) -> String {
    info!("Generating rss channel");
    let mut channel = ChannelBuilder::default()
        .title(&opts.title)
        .link(&opts.site_address)
        .description(format!("Aggregated Liveboat rss feed for {}", opts.title))
        .build();
    let mut items = Vec::new();
    for feed in feeds {
        if feed.is_hidden() {
            continue;
        }
        for feed_item in feed.clone().items {
            items.push(feed_item)
        }
    }
    items.sort_by_key(|w| Reverse(w.date()));
    for item in items {
        channel
            .items
            .push(item.to_rss_item(opts.include_article_content_in_rss_feeds))
    }
    return channel.to_string();
}

/// Generate OPML file.
pub fn generate_opml(opts: &Options, feeds: &Vec<Feed>) -> String {
    let mut tagged_feeds: HashMap<String, Vec<&Feed>> = HashMap::new();
    let mut bare_feeds: Vec<&Feed> = Vec::new();
    for f in feeds {
        if f.is_query_feed() {
            continue
        }
        if f.tags.len() > 0 {
            for t in f.tags.clone() {
                if tagged_feeds.contains_key(&t) {
                    let ff = tagged_feeds.get_mut(&t.clone()).unwrap();
                    ff.push(f);
                } else {
                    let n = Vec::from([f]);
                    tagged_feeds.insert(t, n);
                }
            }
        } else {
            bare_feeds.push(f)
        }
    }
    let mut op = OPML::default();
    let mut head = Head::default();
    head.title = Some(opts.title.clone());
    head.date_created = Some(Local::now().to_rfc2822());
    head.date_modified = Some(Local::now().to_rfc2822());
    op.head = Some(head);
    let mut body = Body::default();
    for tag in tagged_feeds.keys() {
        let mut o = Outline::default();
        o.title = Some(tag.clone());
        o.text = tag.clone();
        for f in tagged_feeds.get(tag).unwrap() {
            let mut feed_outline = generate_feed_outline(f);
            feed_outline.category = Some(tag.clone());
            o.outlines.push(generate_feed_outline(f))
        }
        body.outlines.push(o);
    }
    for f in bare_feeds {
        body.outlines.push(generate_feed_outline(f))
    }
    op.body = body.clone();
    return op.to_string().unwrap();
}

/// Generate outline instance from feed.
fn generate_feed_outline(f: &Feed) -> Outline {
    let mut feed_outline = Outline::default();
    feed_outline.title = Some(f.display_title().clone());
    feed_outline.text = f.display_title().clone();
    feed_outline.xml_url = Some(f.url().clone());
    feed_outline.html_url = Some(f.feedlink.clone());
    feed_outline.r#type = Some("rss".to_string());
    return feed_outline;
}
