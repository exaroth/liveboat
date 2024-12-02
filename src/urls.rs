use std::fs::read_to_string;
use std::path::Path;
use log::info;

use libnewsboat::matcher::Matcher;
use libnewsboat::utils as libutils;

#[derive(Debug, Clone)]
pub struct URLFeed {
    pub url: String,
    pub tags: Vec<String>,
    pub hidden: bool,
    pub title_override: Option<String>,
}

pub struct QueryFeed {
    pub title: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct UrlReader {
    lines: Vec<String>,
}

impl UrlReader {
    pub fn init(url_fpath: &Path) -> UrlReader {
        info!("Initializing reader");
        let mut u = UrlReader { lines: Vec::new() };
        u.read(url_fpath);
        return u;
    }

    fn read(&mut self, path: &Path) {
        let mut result: Vec<String> = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            info!("Reading line {}", line);
            if line.is_empty() || line.starts_with("#") {
                info!("Ignoring");
                continue;
            }
            result.push(String::from(line));
        }
        self.lines = result;
    }

    fn get_http_feed(&self, tokens: &Vec<String>) -> URLFeed {
        info!("Processing http feed");
        let mut feed = URLFeed {
            url: String::from(&tokens[0]),
            tags: Vec::new(),
            hidden: false,
            title_override: None,
        };
        info!("Default feed: {}", format!("{:?}", feed));
        let l = tokens.len();
        if l > 1 {
            for i in 1..l {
                info!("Processing token: {}", tokens[i]);
                if tokens[i].starts_with("~") {
                    info!("Title token found");
                    let mut t_override = tokens[i].clone();
                    t_override.remove(0);
                    feed.title_override = Some(t_override);
                    continue;
                }
                if tokens[i] == "!" {
                    info!("Setting feed to hidden");
                    feed.hidden = true;
                    continue;
                }
                info!("Adding tag");
                feed.tags.push(String::from(&tokens[i]))
            }
        }
        info!("URL feed after: {}", format!("{:?}", feed));
        return feed;
    }

    pub fn get_url_feeds(&self) -> Vec<URLFeed> {
        info!("Retrieving url feeds from url file");
        let mut result = Vec::new();
        for line in &self.lines {
            info!("Line: {}", line);
            let tokens = libutils::tokenize_quoted(line.as_str(), " \r\n\t");
            info!("Tokens: {}", format!("{:?}", tokens));
            if tokens.is_empty() {
                info!("Is empty, Skipping");
                continue;
            }
            if libutils::is_special_url(tokens[0].as_str()) {
                info!("Is special, skipping");
                continue;
            }
            let feed = self.get_http_feed(&tokens);
            result.push(feed);
        }
        result
    }

    pub fn get_query_urls(&self) -> Result<Vec<QueryFeed>, String> {
        info!("Retrieving query urls");
        let mut results = Vec::new();
        for line in &self.lines {
            let tokens = libutils::tokenize_quoted(line.as_str(), " \r\n\t");
            if tokens.is_empty() {
                continue;
            }
            if !libutils::is_query_url(tokens[0].as_str()) {
                info!("Skipping line: {}", format!("{:?}", tokens));
                continue;
            }
            let parts = libutils::tokenize_quoted(tokens[0].as_str(), ":");
            info!("Parts are: {}", format!("{:?}", parts));
            if parts.len() < 3 {
                return Err(format!("Invalid query found: {}", line));
            }
            let filter_s = &parts[2];
            info!("Matching against: {}", filter_s);
            match Matcher::parse(filter_s) {
                Ok(r) => results.push(QueryFeed {
                    title: parts[1].clone(),
                    matcher: r,
                }),
                Err(e) => return Err(e),
            };
        }
        Ok(results)
    }
}
