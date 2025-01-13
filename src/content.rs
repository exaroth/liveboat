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

/// Prettify article content
pub fn process_article_content(
    url_string: &String,
    original_content: &mut String,
    options: &Options,
) -> Result<(String, String, usize)> {
    let mut scrape = false;
    let mut url = Url::parse(url_string)?;
    let mut content_length = 0;
    if options.scrape_reddit_links {
        let r_res = get_reddit_direct_link(&url, &original_content);
        if r_res.is_some() {
            url = r_res.unwrap();
            scrape = true;
        }
    }
    let mut content = String::new();
    let result: Result<extractor::Product, readability_liveboat::error::Error>;
    if scrape {
        result = extractor::scrape(&url.as_str());
    } else {
        result = extractor::extract(&mut original_content.as_bytes(), &url);
    }
    if result.is_ok() {
        let t = result.unwrap();
        content = t.content;
        content_length = t.text.len();
    }

    Ok((content.trim().to_string(), url.to_string(), content_length))
}
