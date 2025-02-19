use log::info;
use std::cmp::Reverse;
use std::collections::HashMap;

use opml::{Body, Head, Outline, OPML};
use rss::{Channel, ChannelBuilder};
use url::Url;

use crate::feed::Feed;
use crate::opts::Options;
use crate::utils::now;

/// Generate RSS channel for the liveboat page.
pub fn generate_rss_channel(
    opts: &Options,
    feeds: &Vec<Feed>,
    general_channel: bool,
) -> String {
    info!("Generating rss channel");
    let mut channel: Channel;
    if general_channel {
        channel = ChannelBuilder::default()
            .title(&opts.title)
            .description(String::from("Liveboat RSS Feed"))
            .link(opts.site_url.clone())
            .build();
    } else {
        channel = ChannelBuilder::default()
            .title(feeds[0].display_title())
            .link(opts.site_url.clone())
            .build()
    }
    let mut items = Vec::new();
    let mut article_guids: Vec<i64> = Vec::new();
    for feed in feeds {
        let mut fc = feed.clone();
        fc.truncate_items();

        for feed_item in fc.items {
            if article_guids.iter().any(|i| i == &feed_item.guid()) {
                continue;
            };
            article_guids.push(feed_item.guid().clone());
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
pub fn generate_opml(
    opts: &Options,
    feeds: &Vec<Feed>,
    site_url: &Url,
) -> String {
    let mut tagged_feeds: HashMap<String, Vec<&Feed>> = HashMap::new();
    let mut bare_feeds: Vec<&Feed> = Vec::new();
    for f in feeds {
        if f.is_hidden() {
            continue;
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
    head.date_created = Some(now().to_rfc2822());
    head.date_modified = Some(now().to_rfc2822());
    op.head = Some(head);
    let mut body = Body::default();
    for tag in tagged_feeds.keys() {
        let mut o = Outline::default();
        o.title = Some(tag.clone());
        o.text = tag.clone();
        for f in tagged_feeds.get(tag).unwrap() {
            let mut feed_outline = generate_feed_outline(f, site_url);
            feed_outline.category = Some(tag.clone());
            o.outlines.push(feed_outline)
        }
        body.outlines.push(o);
    }
    for f in bare_feeds {
        body.outlines.push(generate_feed_outline(f, site_url))
    }
    op.body = body.clone();
    return op.to_string().unwrap();
}

/// Generate outline instance from feed.
fn generate_feed_outline(f: &Feed, site_url: &Url) -> Outline {
    let mut feed_outline = Outline::default();
    feed_outline.title = Some(f.display_title().clone());
    feed_outline.text = f.display_title().clone();
    feed_outline.r#type = Some("rss".to_string());
    if f.is_query_feed() {
        let channel_url = site_url
            .join(format!("channels/{}.xml", f.id()).as_str())
            .unwrap();
        feed_outline.xml_url = Some(channel_url.to_string());
        feed_outline.html_url = Some(site_url.to_string());
    } else {
        feed_outline.xml_url = Some(f.url().clone());
        feed_outline.html_url = Some(f.feedlink().clone());
    }
    return feed_outline;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::feed_item::FeedItem;

    #[test]
    fn test_generating_rss_channel_from_feeds() {
        let mut f1 = Feed::init(
            "www.example.com/rss".to_string(),
            "Test feed 1".to_string(),
            "www.example.com".to_string(),
        );
        f1.items.push(FeedItem::new(
            "item1",
            "http://test1.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 1",
            1,
        ));
        f1.items.push(FeedItem::new(
            "item2",
            "http://test2.com",
            "",
            "exaroth",
            1733100000,
            false,
            "Test content 2",
            2,
        ));
        f1.items.push(FeedItem::new(
            "item3",
            "http://test3.com",
            "",
            "exaroth",
            1733200000,
            true,
            "Test content 3",
            3,
        ));
        f1.update_with_url_data(
            Vec::from(["dev".to_string(), "tech".to_string()]),
            false,
            Some("Title override".to_string()),
            1,
        );
        let mut f2 = Feed::init_query_feed("Query feed".to_string(), 1);
        f2.items.push(FeedItem::new(
            "item4",
            "http://test4.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 4",
            4,
        ));
        let mut f3 = Feed::init(
            "www.example.com/rss".to_string(),
            "Test feed 1".to_string(),
            "www.example.com".to_string(),
        );
        f3.items.push(FeedItem::new(
            "item5",
            "http://test5.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 5",
            5,
        ));
        f3.update_with_url_data(Vec::new(), true, None, 1);

        let result =
            generate_rss_channel(&Options::default(), &Vec::from([f1, f2, f3]), true);
        assert_eq!(result,  "<?xml version=\"1.0\" encoding=\"utf-8\"?><rss version=\"2.0\"><channel><title>Liveboat feed page</title><link>http://site-url-not-set.io/you-can-set-it-in-liveboat-config</link><description>Liveboat RSS Feed</description><item><title>item3</title><link>http://test3.com</link><author>exaroth</author><pubDate>Tue, 3 Dec 2024 04:26:40 +0000</pubDate></item><item><title>item2</title><link>http://test2.com</link><author>exaroth</author><pubDate>Mon, 2 Dec 2024 00:40:00 +0000</pubDate></item><item><title>item1</title><link>http://test1.com</link><author>exaroth</author><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item><item><title>item4</title><link>http://test4.com</link><author>exaroth</author><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item><item><title>item5</title><link>http://test5.com</link><author>exaroth</author><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item></channel></rss>")
    }

    #[test]
    fn test_generating_rss_with_media() {
        let mut f1 = Feed::init(
            "www.example.com/rss".to_string(),
            "Test feed 1".to_string(),
            "www.example.com".to_string(),
        );
        let mut item = FeedItem::new(
            "item1",
            "http://test1.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 1",
            1,
        );
        item.set_enc_url(String::from("http://www.example.com/test.mp3"));
        item.set_enc_mime(String::from("audio/mp3"));
        f1.items.push(item);
        let result =
            generate_rss_channel(&Options::default(), &Vec::from([f1]), true);
        assert_eq!(result,  "<?xml version=\"1.0\" encoding=\"utf-8\"?><rss version=\"2.0\"><channel><title>Liveboat feed page</title><link>http://site-url-not-set.io/you-can-set-it-in-liveboat-config</link><description>Liveboat RSS Feed</description><item><title>item1</title><link>http://test1.com</link><author>exaroth</author><enclosure url=\"http://www.example.com/test.mp3\" length=\"\" type=\"audio/mp3\"/><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item></channel></rss>")
    }

    #[test]
    fn test_generating_channel_feed() {
        let mut f1 = Feed::init(
            "www.example.com/rss".to_string(),
            "Test feed 1".to_string(),
            "www.example.com".to_string(),
        );
        let mut item = FeedItem::new(
            "item1",
            "http://test1.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 1",
            1,
        );
        f1.items.push(item);
        let result =
            generate_rss_channel(&Options::default(), &Vec::from([f1]), false);
        assert_eq!(result,  "<?xml version=\"1.0\" encoding=\"utf-8\"?><rss version=\"2.0\"><channel><title>Test feed 1</title><link>http://site-url-not-set.io/you-can-set-it-in-liveboat-config</link><description></description><item><title>item1</title><link>http://test1.com</link><author>exaroth</author><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item></channel></rss>")
    }

    #[test]
    fn test_generating_rss_with_query_feeds() {
        let mut f = Feed::init(
            "www.example.com/rss".to_string(),
            "Test feed".to_string(),
            "www.example.com".to_string(),
        );
        let i1 = FeedItem::new(
            "item1",
            "http://test1.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 1",
            1,
        );
        f.items.push(i1.clone());
        let mut qf = Feed::init_query_feed(String::from("Test query feed"), 1);
        qf.items.push(i1.clone());
        let result = generate_rss_channel(
            &Options::default(),
            &Vec::from([f, qf]),
            true,
        );
        assert_eq!(result,  "<?xml version=\"1.0\" encoding=\"utf-8\"?><rss version=\"2.0\"><channel><title>Liveboat feed page</title><link>http://site-url-not-set.io/you-can-set-it-in-liveboat-config</link><description>Liveboat RSS Feed</description><item><title>item1</title><link>http://test1.com</link><author>exaroth</author><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item></channel></rss>")
    }
    #[test]
    fn test_generting_rss_with_hidden_feeds() {
        let mut f = Feed::init(
            "www.example.com/rss".to_string(),
            "Test feed".to_string(),
            "www.example.com".to_string(),
        );
        f.update_with_url_data(Vec::new(), true, None, 1);
        let i1 = FeedItem::new(
            "item1",
            "http://test1.com",
            "",
            "exaroth",
            1733000000,
            false,
            "Test content 1",
            1,
        );
        f.items.push(i1);
        let result =
            generate_rss_channel(&Options::default(), &Vec::from([f]), true);
        assert_eq!(result,  "<?xml version=\"1.0\" encoding=\"utf-8\"?><rss version=\"2.0\"><channel><title>Liveboat feed page</title><link>http://site-url-not-set.io/you-can-set-it-in-liveboat-config</link><description>Liveboat RSS Feed</description><item><title>item1</title><link>http://test1.com</link><author>exaroth</author><pubDate>Sat, 30 Nov 2024 20:53:20 +0000</pubDate></item></channel></rss>")
    }

    #[test]
    fn test_generating_opml_from_feeds() {
        let f1 = Feed::init(
            "www.test1.com/rss".to_string(),
            "Test feed 1".to_string(),
            "www.example.com".to_string(),
        );
        let mut f2 = Feed::init(
            "www.test2.com/rss".to_string(),
            "Test feed 2".to_string(),
            "www.example2.com".to_string(),
        );
        f2.tags.push("test".to_string());
        let mut f3 = Feed::init_query_feed("Query feed".to_string(), 1);
        let opts = Options::default();
        let result = generate_opml(
            &opts,
            &Vec::from([f1, f2, f3]),
            &Url::parse("http://www.example.com").unwrap(),
        );
        assert_eq!("<opml version=\"2.0\"><head><title>Liveboat feed page</title><dateCreated>Thu, 12 Dec 2024 03:42:54 +0000</dateCreated><dateModified>Thu, 12 Dec 2024 03:42:54 +0000</dateModified></head><body><outline text=\"test\" title=\"test\"><outline text=\"Test feed 2\" type=\"rss\" category=\"test\" xmlUrl=\"www.test2.com/rss\" htmlUrl=\"www.example2.com\" title=\"Test feed 2\"/></outline><outline text=\"Test feed 1\" type=\"rss\" xmlUrl=\"www.test1.com/rss\" htmlUrl=\"www.example.com\" title=\"Test feed 1\"/><outline text=\"Query feed\" type=\"rss\" xmlUrl=\"http://www.example.com/channels/5aSKHsoqmmoCnw.xml\" htmlUrl=\"http://www.example.com/\" title=\"Query feed\"/></body></opml>", result)
    }
}
