use log::info;
use rss::ChannelBuilder;

use crate::feed::Feed;
use crate::opts::Options;

/// Generate RSS channel for the liveboat page.
pub fn generate_rss_channel(opts: &Options, feeds: &Vec<Feed>) -> String {
    info!("Generating rss channel");
    let mut channel = ChannelBuilder::default()
        .title(&opts.title)
        // TODO: add url to options
        .link("http://example.com")
        .description(format!("Aggregated liveboat rss feed for {}", opts.title))
        .build();
    let mut items = Vec::new();
    for feed in feeds {
        for feed_item in feed.clone().items {
            items.push(feed_item)
        }
    }
    items.sort_by(|a, b| a.date().cmp(&b.date()));
    for item in items {
        channel.items.push(item.to_rss_item())
    }
    return channel.to_string()
}
