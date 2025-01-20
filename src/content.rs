use log::info;

use anyhow::Result;
use readability_liveboat::extractor;
use regex::Regex;
use url::Url;

use crate::opts::Options;

const REDDIT_SELF_REFERENTIAL_DOMAINS: &[&str] = &[
    "www.reddit.com",
    "i.redd.it",
    "old.reddit.com",
    "new.reddit.com",
];

const SCRAPE_EXCLUDED_DOMAINS: &[&str] =
    &["github.com", "github.io", "bloomberg.com", "youtube.com"];

#[derive(Debug)]
pub struct ContentProcessingResult {
    pub content: String,
    pub text: String,
    pub url: String,
    pub comments_url: Option<String>,
}

impl ContentProcessingResult {
    /// Return length of the content, this
    /// will return length of the actual
    /// text contained excluding any HTML tags.
    pub fn content_length(&self) -> usize {
        return self.text.len();
    }

    fn default(url: String) -> ContentProcessingResult {
        return ContentProcessingResult {
            content: String::new(),
            text: String::new(),
            url: url,
            comments_url: None,
        };
    }
}

/// Process article content, filtering all extraneous data
/// and retrieving comments urls when necessary
pub fn process_article_content(
    url_string: &String,
    feedlink: &String,
    feed_url: &String,
    original_content: &mut String,
    options: &Options,
) -> Result<ContentProcessingResult> {
    let mut scrape = false;
    let mut result = ContentProcessingResult::default(url_string.clone());
    let mut url = Url::parse(url_string)?;
    if options.scrape_reddit_links {
        let r_res = get_reddit_direct_link(&url, &original_content);
        if r_res.is_some() {
            result.comments_url = Some(url_string.clone());
            url = r_res.unwrap();
            result.url = url.to_string();
            scrape = true;
        }
    }
    if options.scrape_hn_links && !scrape {
        let h_res = get_hn_links(&url, feedlink, feed_url, original_content)?;
        if h_res.is_some() {
            let h_parts = h_res.unwrap();
            result.comments_url = Some(h_parts.0);
            scrape = h_parts.1
        }
    }
    if SCRAPE_EXCLUDED_DOMAINS
        .iter()
        .any(|d| url.host().unwrap().to_string().contains(d))
    {
        info!("excluding domain from scraping {:?}", url_string);
        scrape = false;
    }
    let extract_result: Result<extractor::Product, readability_liveboat::error::Error>;
    if scrape {
        extract_result = extractor::scrape(&result.url.as_str());
    } else {
        extract_result = extractor::extract(&mut original_content.as_bytes(), &url);
    }
    if extract_result.is_ok() {
        let t = extract_result.unwrap();
        result.content = t.content;
        result.text = t.text;
    }

    Ok(result)
}

/// Dispatch processing for HN related feeds (hnrss.org and
/// native news.ycombinator.com native feed), retrieving
/// comment urls and deciding whether to dispatch scrape
/// processing for associated urls.
fn get_hn_links(
    url: &Url,
    feedlink: &String,
    feed_url: &String,
    content: &String,
) -> Result<Option<(String, bool)>> {
    // Don't process self referential links for now.
    if url.path() == "news.ycombinator.com" {
        return Ok(Some((url.to_string(), false)));
    }
    let f_url = Url::parse(feed_url)?;
    let host_opt = f_url.host();
    if host_opt.is_none() {
        return Ok(None);
    }
    let host = host_opt.unwrap().to_string();
    if host == "hnrss.org" {
        return Ok(get_hnrss_org_links(&Url::parse(feedlink)?, content));
    }
    if host == "news.ycombinator.com" {
        return Ok(get_native_hn_links(content));
    }
    return Ok(None);
}

/// Fetch direct link from Reddits RSS content.
fn get_reddit_direct_link(url: &Url, content: &String) -> Option<Url> {
    let host = url.host();
    if host.is_none() || host.unwrap().to_string() != String::from("www.reddit.com") {
        return None;
    }
    let re = Regex::new(r#"<a.*href\s?=['"]*(?<href>[^'"]*)[^>]*>\[link\]<\/a>"#).unwrap();
    let cap_result = re.captures(content);
    if cap_result.is_none() {
        info!("No matching links in response response");
        return None;
    }
    let caps = cap_result.unwrap();
    if caps.len() != 2 {
        info!("Invalid number of matches in response");
        return None;
    }
    let res = Url::parse(&caps[1]);
    if res.is_err() {
        info!("Cant parse url: {}", &caps[0]);
        return None;
    }
    let r_url = res.unwrap();
    // Skip fetching link to comments
    let host = r_url.host();
    if host.is_none() {
        info!("No host found in {}", &caps[0]);
        return None;
    }
    let domain = host.unwrap().to_string();
    let self_referential = REDDIT_SELF_REFERENTIAL_DOMAINS.iter().any(|d| d == &domain);
    if self_referential {
        info!("Self referential reddit link found, skipping");
        return None;
    }

    return Some(r_url);
}

/// List of hn sections to be excluded from scraping.
const SCRAPE_EXCLUDED_HN_SECTIONS: &[&str] = &["/ask"];

/// Retrieve comments url from hnrss.org links and check if the page should
/// be scraped based on the HN section.
fn get_hnrss_org_links(feedlink: &Url, content: &String) -> Option<(String, bool)> {
    // Matching on:
    // <p>Comments URL: <a href="?(<url>")>
    let re = Regex::new(r#"<p>Comments URL: <a.*href\s?=['"]*(?<href>[^'"]*)[^>]*>"#).unwrap();
    let cap_result = re.captures(content);
    if cap_result.is_none() {
        info!("No matching links in response response");
        return None;
    }
    let caps = cap_result.unwrap();
    if caps.len() != 2 {
        info!("Invalid number of matches in response");
        return None;
    }
    if SCRAPE_EXCLUDED_HN_SECTIONS
        .iter()
        .any(|s| s == &feedlink.path())
    {
        return Some((caps[1].to_string(), false));
    }
    return Some((caps[1].to_string(), true));
}

/// Retrieve comment link from native hn rss feed
/// (news.ycombinator.com//rss)
fn get_native_hn_links(content: &String) -> Option<(String, bool)> {
    let re = Regex::new(r#"<a.*href\s?=['"]*(?<href>[^'"]*)[^>]*>"#).unwrap();
    let cap_result = re.captures(content);
    if cap_result.is_none() {
        info!("No matching links in response response");
        return None;
    }
    let caps = cap_result.unwrap();
    if caps.len() != 2 {
        info!("Invalid number of matches in response");
        return None;
    }
    // Note: for native hn feeds we will dispatch scrape for all links
    return Some((caps[1].to_string(), true))
}
