use std::fs::read_to_string;
use std::path::Path;

use libnewsboat::utils as libutils;
use libnewsboat::matcher::Matcher;

#[derive(Debug, Clone)]
pub struct URLFeed {
    pub url: String,
    pub tags: Vec<String>,
    pub hidden: bool,
    pub title_override: Option<String>
}

pub struct QueryFeed {
    pub title: String,
    pub matcher: Matcher,
}


#[derive(Clone, Debug)]
pub struct UrlReader {
    pub lines: Vec<String>,
}

impl UrlReader {
    pub fn init(url_fpath: &Path) -> UrlReader {
        let mut u = UrlReader { lines: Vec::new() };
        u.read(url_fpath);
        return u;
    }

    fn read(&mut self, path: &Path) {
        let mut result: Vec<String> = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            result.push(String::from(line));
        }
        self.lines = result;
    }

    fn get_http_feed(&self, tokens: &Vec<String>) -> URLFeed {
        let mut feed = URLFeed {
            url: String::from(&tokens[0]),
            tags: Vec::new(),
            hidden: false,
            title_override: None,
        };
        let l = tokens.len();
        if l > 1 {
            for i in 1..l {
                if tokens[i].starts_with("~") {
                    let mut t_override = tokens[i].clone();
                    t_override.remove(0);
                    feed.title_override = Some(t_override);
                    continue;
                }
                if tokens[i] == "!" {
                    feed.hidden = true;
                    continue;
                }
                feed.tags.push(String::from(&tokens[i]))
            }
        }
        return feed;
    }

    pub fn get_url_feeds(&self) -> Vec<URLFeed> {
        let mut result = Vec::new();
        for line in &self.lines {
            let tokens = libutils::tokenize_quoted(line.as_str(), " \r\n\t");
            if tokens.is_empty() {
                continue;
            }
            if libutils::is_special_url(tokens[0].as_str()) {
                continue;
            }
            let feed = self.get_http_feed(&tokens);
            result.push(feed);
        }
        result
    }

    pub fn get_query_urls(&self) -> Result<Vec<QueryFeed>, String> {
        let mut results = Vec::new();
        for line in &self.lines {
            let tokens = libutils::tokenize_quoted(line.as_str(), " \r\n\t");
            if tokens.is_empty() {
                continue;
            }
            if !libutils::is_query_url(tokens[0].as_str()) {
                continue;
            }
            let parts = libutils::tokenize_quoted(tokens[0].as_str(), ":");
            if parts.len() < 3 {
                return Err(format!("Invalid query found: {}", line))
            }
            let filter_s = &parts[2];
            match Matcher::parse(filter_s) {
                Ok(r) => results.push(QueryFeed{title: parts[1].clone(), matcher: r}),
                Err(e) => return Err(e)
            };
        }
        Ok(results)
    }
}
