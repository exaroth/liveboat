use log::info;
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
        for feed_item in feed.clone().items {
            items.push(feed_item)
        }
    }
    items.sort_by(|a, b| a.date().cmp(&b.date()));
    for item in items {
        channel
            .items
            .push(item.to_rss_item(opts.include_article_content_in_rss_feeds))
    }
    return channel.to_string();
}
