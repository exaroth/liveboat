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
        return self.text.len()
    }

    fn default(url: String) -> ContentProcessingResult {
        return ContentProcessingResult{
            content: String::new(),
            text: String::new(),
            url: url,
            comments_url: None,
        }
    }
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

/// Process article content, filtering all extraneous data
/// and retrieving comments urls when necessary
pub fn process_article_content(
    url_string: &String,
    original_content: &mut String,
    options: &Options,
) -> Result<ContentProcessingResult> {
    let mut scrape = false;
    let mut result = ContentProcessingResult::default(url_string.clone());
    let url = Url::parse(url_string)?;
    if options.scrape_reddit_links {
        let r_res = get_reddit_direct_link(&url, &original_content);
        if r_res.is_some() {
            result.comments_url = Some(url_string.clone());
            result.url = r_res.unwrap().to_string();
            scrape = true;
        }
    } else if options.scrape_hn_links{
        println!("{:?}", "depro");
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
        result.text = t.text
    }

    Ok(result)
}
