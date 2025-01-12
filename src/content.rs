use log::info;

use anyhow::Result;
use readability::extractor;
use regex::Regex;
use url::Url;

/// Fetch direct link from Reddits RSS content.
pub fn get_reddit_direct_link(url: &Url, content: &String) -> Option<Url> {
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
    if host.unwrap().to_string().as_str() == "www.reddit.com" {
        info!("Self referential reddit link found, skipping");
        return None;
    }

    return Some(r_url);
}

/// Prettify article content
pub fn process_article_content(url_string: &String, content: &mut String) -> Result<String> {
    let mut scrape = false;
    let mut url = Url::parse(url_string)?;
    // TODO: add option
    let r_res = get_reddit_direct_link(&url, &content);
    if r_res.is_some() {
        url = r_res.unwrap();
        scrape = true;
    }
    let result: extractor::Product;
    if scrape {
        result = extractor::scrape(&url.as_str())?;
    } else {
        result = extractor::extract(&mut content.as_bytes(), &url)?;
    }

    let mut content = result.content;
    content = content.trim().to_string();
    Ok(content)
}
