/// This module contains logic related to processing Newsboat urls files.

use log::info;

use libnewsboat::matcher::Matcher;
use libnewsboat::utils as libutils;

/// Representation of single url based feed,
/// this will also include file based feeds.
#[derive(Debug, Clone)]
pub struct URLFeed {
    pub url: String,
    pub tags: Vec<String>,
    pub hidden: bool,
    pub title_override: Option<String>,
    pub line_no: usize,
}

/// Representation of query based feed.
pub struct QueryFeed {
    pub title: String,
    pub matcher: Matcher,
    pub line_no: usize,
}

/// Module used for operating on the 
/// Newsboat urls file.
#[derive(Clone, Debug)]
pub struct UrlReader {
    lines: Vec<String>,
}

impl UrlReader {
    pub fn init(url_file: String) -> UrlReader {
        info!("Initializing reader");
        let mut u = UrlReader { lines: Vec::new() };
        u.read(url_file);
        return u;
    }
    
    /// Read the lines of the urls file, filtering out 
    /// comments and empty lines.
    fn read(&mut self, url_file: String) {
        let mut result: Vec<String> = Vec::new();
        for line in url_file.lines() {
            info!("Reading line {}", line);
            if line.is_empty() || line.starts_with("#") {
                info!("Ignoring");
                continue;
            }
            result.push(String::from(line));
        }
        self.lines = result;
    }
    
    /// Process tokens associated with single url feed.
    fn get_http_feed(&self, tokens: &Vec<String>, line_no: usize) -> URLFeed {
        info!("Processing http feed");
        let mut feed = URLFeed {
            url: String::from(&tokens[0]),
            tags: Vec::new(),
            hidden: false,
            title_override: None,
            line_no: line_no,
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
    

    /// Fetch all url feeds as defined in urls file.
    pub fn get_url_feeds(&self) -> Vec<URLFeed> {
        info!("Retrieving url feeds from url file");
        let mut result = Vec::new();
        let linel = self.lines.len();
        for line_no in 0..linel {
            let line = &self.lines[line_no];
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
            let feed = self.get_http_feed(&tokens, line_no);
            result.push(feed);
        }
        result
    }
    
    /// Fetch all query urls as defined in urls file.
    pub fn get_query_urls(&self) -> Result<Vec<QueryFeed>, String> {
        info!("Retrieving query urls");
        let mut results = Vec::new();
        let linel = self.lines.len();
        for line_no in 0..linel {
            let line = &self.lines[line_no];
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
                    line_no: line_no
                }),
                Err(e) => return Err(e),
            };
        }
        Ok(results)
    }
}
